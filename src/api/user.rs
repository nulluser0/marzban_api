//! # User API Category

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user::{
            UserCreate, UserModify, UserResponse, UserStatus, UserUsagesResponse, UsersResponse,
            UsersUsagesResponse,
        },
    },
};

// Custom struct for query params in get users
#[derive(Serialize)]
pub struct GetUsersQueryParams {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub username: Option<Vec<String>>,
    pub status: Option<UserStatus>,
    pub sort: Option<String>,
}

impl MarzbanAPIClient {
    /// `POST /api/user`
    ///
    /// Add a new user
    ///
    /// - **username**: 3 to 32 characters, can include a-z, 0-9, and underscores.
    /// - **status**: User's status, defaults to `active``. Special rules if `on_hold``.
    /// - **expire**: UTC timestamp for account expiration. Use `0` for unlimited.
    /// - **data_limit**: Max data usage in bytes (e.g., `1073741824` for 1GB). `0`` means unlimited.
    /// - **data_limit_reset_strategy**: Defines how/if data limit resets. `no_reset` means it never resets.
    /// - **proxies**: Dictionary of protocol settings (e.g., `vmess`, `vless`).
    /// - **inbounds**: Dictionary of protocol tags to specify inbound connections.
    /// - **note**: Optional text field for additional user information or notes.
    /// - **on_hold_timeout**: UTC timestamp when `on_hold` status should start or end.
    /// - **on_hold_expire_duration**: Duration (in seconds) for how long the user should stay in `on_hold` status.
    pub async fn add_user(&self, new_user: UserCreate) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
                    .json(&new_user)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "User already exists".to_string(),
            )),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/user/{username}`
    ///
    /// Get user information
    pub async fn get_user(&self, username: impl Into<String>) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}", self.inner.base_url, username.into());
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `PUT /api/user/{username}`
    ///
    /// Modify an existing user
    ///
    /// - **username**: Cannot be changed. Used to identify the user.
    /// - **status**: User's new status. Can be 'active', 'disabled', 'on_hold', 'limited', or 'expired'.
    /// - **expire**: UTC timestamp for new account expiration. Set to `0`` for unlimited, `null` for no change.
    /// - **data_limit**: New max data usage in bytes (e.g., `1073741824` for 1GB). Set to `0` for unlimited, `null` for no change.
    /// - **data_limit_reset_strategy**: New strategy for data limit reset. Options include 'daily', 'weekly', 'monthly', or 'no_reset'.
    /// - **proxies**: Dictionary of new protocol settings (e.g., `vmess`, `vless`). Empty dictionary means no change.
    /// - **inbounds**: Dictionary of new protocol tags to specify inbound connections. Empty dictionary means no change.
    /// - **note**: New optional text for additional user information or notes. `null` means no change.
    /// - **on_hold_timeout**: New UTC timestamp for when `on_hold` status should start or end. Only applicable if status is changed to 'on_hold'.
    /// - **on_hold_expire_duration**: New duration (in seconds) for how long the user should stay in `on_hold` status. Only applicable if status is changed to 'on_hold'.
    ///
    /// Note: Fields set to `null` or omitted will not be modified.
    pub async fn modify_user(
        &self,
        username: impl AsRef<str>,
        body: UserModify,
    ) -> Result<UserResponse, ApiError> {
        let url = format!("{}/api/user/{}", self.inner.base_url, username.as_ref());
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `DELETE /api/user/{username}`
    ///
    /// Remove a user
    pub async fn delete_user(&self, username: impl AsRef<str>) -> Result<String, ApiError> {
        let url = format!("{}/api/user/{}", self.inner.base_url, username.as_ref());
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::DELETE, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/user/{username}/reset`
    ///
    /// Reset user data usage
    pub async fn reset_user_data_usage(
        &self,
        username: impl AsRef<str>,
    ) -> Result<UserResponse, ApiError> {
        let url = format!(
            "{}/api/user/{}/reset",
            self.inner.base_url,
            username.as_ref()
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "User already exists".to_string(),
            )),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/user/{username}/renew`
    ///
    /// Revoke users subscription (Subscription link and proxies)
    pub async fn revoke_user_subscription(
        &self,
        username: impl AsRef<str>,
    ) -> Result<UserResponse, ApiError> {
        let url = format!(
            "{}/api/user/{}/revoke_sub",
            self.inner.base_url,
            username.as_ref()
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/users`
    ///
    /// Get all users
    pub async fn get_users(
        &self,
        query_params: GetUsersQueryParams,
    ) -> Result<UsersResponse, ApiError> {
        let url = format!("{}/api/users", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&query_params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UsersResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/users/reset`
    ///
    /// Reset all users data usage
    pub async fn reset_all_users_data_usage(&self) -> Result<String, ApiError> {
        let url = format!("{}/api/users/reset", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/user/{username}/usage`
    ///
    /// Get users usage
    pub async fn get_user_usage(
        &self,
        username: impl AsRef<str>,
        start: Option<impl Into<String>>,
        end: Option<impl Into<String>>,
    ) -> Result<UserUsagesResponse, ApiError> {
        let url = format!(
            "{}/api/user/{}/usage",
            self.inner.base_url,
            username.as_ref()
        );
        let mut params = Vec::new();
        if let Some(value) = start {
            params.push(("start", value.into()))
        }
        if let Some(value) = end {
            params.push(("end", value.into()))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserUsagesResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/users/usage`
    ///
    /// Get all users usage
    ///
    /// # Parameters
    ///
    /// - `start` - The start date for the range.
    /// - `end` - The end date for the range.
    /// - `admin` - The users which are owned by the array of admins.
    pub async fn get_all_users_usage(
        &self,
        start: Option<impl Into<String>>,
        end: Option<impl Into<String>>,
        admin: Option<Vec<impl Into<String>>>,
    ) -> Result<UsersUsagesResponse, ApiError> {
        let url = format!("{}/api/users/usage", self.inner.base_url);
        let mut params = Vec::new();
        if let Some(value) = start {
            params.push(("start", value.into()))
        }
        if let Some(value) = end {
            params.push(("end", value.into()))
        }
        if let Some(value) = admin {
            params.push((
                "admin",
                value
                    .into_iter()
                    .map(|x| {
                        let x: String = x.into();
                        x
                    })
                    .collect::<Vec<_>>()
                    .join(","),
            ))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UsersUsagesResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `PUT /api/user/{username}/set-owner`
    ///
    /// Set a new owner (admin) for a user.
    ///
    /// # Parameters
    ///
    /// - `admin_username` - The username of the new owner.
    pub async fn set_owner_of_user(
        &self,
        username: impl AsRef<str>,
        admin_username: impl Into<String>,
    ) -> Result<UserResponse, ApiError> {
        let admin_username = admin_username.into();
        let url = format!(
            "{}/api/user/{}/set-owner",
            self.inner.base_url,
            username.as_ref()
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&admin_username)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("User not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/users/expired`
    ///
    /// Get users who have expired within the specified date range.
    ///
    /// - **expired_after** UTC datetime (optional)
    /// - **expired_before** UTC datetime (optional)
    /// - At least one of expired_after or expired_before must be provided for filtering
    /// - If both are omitted, returns all expired users
    pub async fn get_expired_users(
        &self,
        expired_before: Option<DateTime<Utc>>,
        expired_after: Option<DateTime<Utc>>,
    ) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/users/expired", self.inner.base_url);
        let mut params = Vec::new();
        if let Some(value) = expired_before {
            params.push(("expired_before", value))
        }
        if let Some(value) = expired_after {
            params.push(("expired_after", value))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<String>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `DELETE /api/users/expired`
    ///
    /// Delete users who have expired within the specified date range.
    ///
    /// - **expired_after** UTC datetime (optional)
    /// - **expired_before** UTC datetime (optional)
    /// - At least one of expired_after or expired_before must be provided
    pub async fn delete_expired_users(
        &self,
        expired_before: Option<DateTime<Utc>>,
        expired_after: Option<DateTime<Utc>>,
    ) -> Result<Vec<String>, ApiError> {
        let url = format!("{}/api/users/expired", self.inner.base_url);
        let mut params = Vec::new();
        if let Some(value) = expired_before {
            params.push(("expired_before", value))
        }
        if let Some(value) = expired_after {
            params.push(("expired_after", value))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::DELETE, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<String>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }
}
