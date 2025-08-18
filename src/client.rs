//! # Marzban API Client
//!
//! This module contains the API client for the Marzban API.

use std::{fmt::Debug, future::Future, sync::Arc};

use reqwest::{Client, IntoUrl, Response, StatusCode};
use tokio::sync::{Mutex, RwLock};

use crate::{error::ApiError, models::auth::BodyAdminTokenApiAdminTokenPost};

/// The Marzban API client.
///
/// This struct contains the base URL for the API, the reqwest client, and the token within the Inner struct.
///
/// You do **not** have to wrap the `Client` in an [`Rc`] or [`Arc`] to **reuse** it,
/// because it already uses an [`Arc`] internally.
#[derive(Debug, Clone)]
pub struct MarzbanAPIClient {
    pub(crate) inner: Arc<MarzbanAPIClientRef>,
}

/// The Marzban API client reference. Contains all the data needed to make requests.
/// This struct is used to allow for thread-safe access to the client and cloning, also making it cheap to clone the outer MarzbanAPIClient struct.
pub(crate) struct MarzbanAPIClientRef {
    pub(crate) base_url: String,
    pub(crate) client: Client,
    pub(crate) token: RwLock<Option<String>>,
    pub(crate) username: RwLock<Option<String>>,
    pub(crate) password: RwLock<Option<String>>,
    pub(crate) refresh_lock: Mutex<()>,
}

impl Debug for MarzbanAPIClientRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MarzbanAPIClient")
            .field("base_url", &self.base_url)
            .field("client", &self.client)
            .field("token", &"*****")
            .field("username", &self.username)
            .field("password", &"*****")
            .finish()
    }
}

impl MarzbanAPIClient {
    /// Create a new Marzban API client with the given base URL.
    pub fn new(base_url: &str) -> Self {
        MarzbanAPIClient {
            inner: MarzbanAPIClientRef {
                base_url: base_url.to_string(),
                client: Client::new(),
                token: RwLock::new(None),
                username: RwLock::new(None),
                password: RwLock::new(None),
                refresh_lock: Mutex::new(()),
            }
            .into(),
        }
    }

    /// Create a new Marzban API client with the given base URL and token.
    pub fn new_with_token(base_url: &str, token: &str) -> Self {
        MarzbanAPIClient {
            inner: MarzbanAPIClientRef {
                base_url: base_url.to_string(),
                client: Client::new(),
                token: RwLock::new(Some(token.to_owned())),
                username: RwLock::new(None),
                password: RwLock::new(None),
                refresh_lock: Mutex::new(()),
            }
            .into(),
        }
    }

    /// Helper function to prepare a request with the given method and URL.
    pub fn prepare_request(
        &self,
        method: reqwest::Method,
        url: impl IntoUrl,
    ) -> reqwest::RequestBuilder {
        self.inner.client.request(method, url)
    }

    /// Attach Authorization if we have a token at send time.
    pub async fn attach_auth(&self, mut rb: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(token) = self.inner.token.read().await.as_ref() {
            rb = rb.bearer_auth(token);
        }
        rb
    }

    /// Send, and on 401 once, refresh token (if creds exist) and retry.
    ///
    /// `make` must rebuild the *same* request (method, url, body, query, headers except auth).
    pub async fn send_with_auth_retry<M, Fut>(&self, make: M) -> Result<Response, ApiError>
    where
        M: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = reqwest::RequestBuilder> + Send,
    {
        // 0 = initial try
        // 1 = retry after seeing someone else might have refreshed (after locking)
        // 2 = retry after we refreshed ourselves
        let mut stage = 0usize;

        loop {
            let resp = self.attach_auth(make().await).await.send().await?;

            if resp.status() != StatusCode::UNAUTHORIZED {
                return Ok(resp);
            }

            match stage {
                0 => {
                    // serialize potential refreshes
                    let _g = self.inner.refresh_lock.lock().await;
                    // someone else could have refreshed already; bump stage and try again
                    stage = 1;
                    continue;
                }
                1 => {
                    // still 401 while holding/after lock -> we refresh
                    self.refresh_token_from_saved_creds().await?;
                    stage = 2;
                    continue;
                }
                _ => {
                    // still 401 after our own refresh → give up
                    return Err(ApiError::ApiResponseError("Unauthorized".into()));
                }
            }
        }
    }

    /// Re-auth using saved username/password and store new token.
    async fn refresh_token_from_saved_creds(&self) -> Result<(), ApiError> {
        let (username, password) = {
            let u = self.inner.username.read().await.clone();
            let p = self.inner.password.read().await.clone();
            (u, p)
        };

        let (username, password) = match (username, password) {
            (Some(u), Some(p)) => (u, p),
            _ => {
                // no saved creds -> nothing we can do
                return Err(ApiError::ApiResponseError(
                    "401 and no saved credentials to refresh token".into(),
                ));
            }
        };

        // call the existing endpoint to get a fresh token
        let body = BodyAdminTokenApiAdminTokenPost {
            username,
            password,
            grant_type: None,
            scope: "".to_string(),
            client_id: None,
            client_secret: None,
        };
        let token = self.admin_token(body).await?;
        let mut token_lock = self.inner.token.write().await;
        *token_lock = Some(token.access_token);
        Ok(())
    }
}
