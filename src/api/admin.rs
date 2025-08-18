//! # Admin API Category

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        admin::{Admin, AdminCreate, AdminModify},
        auth::BodyAdminTokenApiAdminTokenPost,
        errors::HTTPValidationError,
        token::Token,
    },
};

impl MarzbanAPIClient {
    /// `POST /api/admin/token`
    ///
    /// Authenticate an admin and issue a token.
    ///
    /// ## Note
    ///
    /// This method does not store the token in the client.
    ///
    /// If you want to add the token to the client, use [MarzbanAPIClient::authenticate()] instead.
    pub async fn admin_token(
        &self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<Token, ApiError> {
        let url = format!("{}/api/admin/token", self.inner.base_url);
        let response = self
            .prepare_request(reqwest::Method::POST, url)
            .form(&auth)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Token>()
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

    /// `POST /api/admin/token`
    ///
    /// Authenticate an admin and issue a token.
    ///
    /// ## Note
    ///
    /// This method takes in a BodyAdminTokenApiAdminTokenPost, and if auth is successful, stores the returned token into the MarzbanAPIClient struct.
    ///
    /// If you want to retrieve the token without storing it in the client, use [MarzbanAPIClient::admin_token()] instead.
    pub async fn authenticate(
        &self,
        auth: BodyAdminTokenApiAdminTokenPost,
    ) -> Result<(), ApiError> {
        let token = self.admin_token(auth.clone()).await?;
        let mut token_lock = self.inner.token.write().await;
        let mut username_lock = self.inner.username.write().await;
        let mut password_lock = self.inner.password.write().await;
        *token_lock = Some(token.access_token);
        *username_lock = Some(auth.username);
        *password_lock = Some(auth.password);
        Ok(())
    }

    /// `GET /api/admin`
    ///
    /// Retrieve the current authenticated admin.
    pub async fn get_current_admin(&self) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/admin`
    ///
    /// Create a new admin if the current admin has sudo privileges.
    pub async fn create_admin(&self, body: AdminCreate) -> Result<Admin, ApiError> {
        let url = format!("{}/api/admin", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Admin already exists".to_string(),
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

    /// `PUT /api/admin/{admin_username}`
    ///
    /// Modify an existing admin's details.
    pub async fn modify_admin(
        &self,
        admin_username: impl AsRef<str>,
        body: AdminModify,
    ) -> Result<Admin, ApiError> {
        let url = format!(
            "{}/api/admin/{}",
            self.inner.base_url,
            admin_username.as_ref()
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Admin not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `DELETE /api/admin/{admin_username}`
    ///
    /// Remove an admin from the database.
    pub async fn delete_admin(&self, admin_username: impl AsRef<str>) -> Result<Admin, ApiError> {
        let url = format!(
            "{}/api/admin/{}",
            self.inner.base_url,
            admin_username.as_ref()
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::DELETE, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Admin>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Admin not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/admins`
    ///
    /// Fetch a list of admins with optional filters for pagination and username.
    ///
    /// ## Parameters
    ///
    /// - `offset` - The offset of the list.
    /// - `limit` - The limit of the list.
    /// - `username` - The username of the admin.
    pub async fn get_admins(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        username: Option<impl Into<String>>,
    ) -> Result<Vec<Admin>, ApiError> {
        let url = format!("{}/api/admins/", self.inner.base_url);
        let mut params = Vec::new();
        if let Some(value) = offset {
            params.push(("offset", value.to_string()))
        }
        if let Some(value) = limit {
            params.push(("limit", value.to_string()))
        }
        if let Some(value) = username {
            params.push(("username", value.into()))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<Admin>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::UNAUTHORIZED => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
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
