//! # User Template API Category

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user_template::{UserTemplateCreate, UserTemplateModify, UserTemplateResponse},
    },
};

impl MarzbanAPIClient {
    /// `GET /api/user_template`
    ///
    /// Get a list of User Templates with optional pagination
    ///
    /// # Parameters
    ///
    /// - `offset` - Optional offset for pagination
    /// - `limit` - Optional limit for pagination
    pub async fn get_user_templates(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<UserTemplateResponse>, ApiError> {
        let url = format!("{}/api/user_template", self.inner.base_url);
        let mut params = Vec::new();
        if let Some(value) = offset {
            params.push(("offset", value))
        }
        if let Some(value) = limit {
            params.push(("limit", value))
        }

        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
                    .query(&params)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<UserTemplateResponse>>()
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

    /// `POST /api/user_template`
    ///
    /// Add a new user template
    ///
    /// - **name** can be up to 64 characters
    /// - **data_limit** must be in bytes and larger or equal to 0
    /// - **expire_duration** must be in seconds and larger or equat to 0
    /// - **inbounds** dictionary of protocol:inbound_tags, empty means all inbounds
    pub async fn add_user_template(
        &self,
        body: UserTemplateCreate,
    ) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Template by this name already exists".to_string(),
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

    /// `GET /api/user_template/{id}`
    ///
    /// Get User Template information with id
    pub async fn get_user_template(&self, id: i32) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template/{}", self.inner.base_url, id);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
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

    /// `PUT /api/user_template/{id}`
    ///
    /// Modify User Template
    ///
    /// - **name** can be up to 64 characters
    /// - **data_limit** must be in bytes and larger or equal to 0
    /// - **expire_duration** must be in seconds and larger or equat to 0
    /// - **inbounds** dictionary of protocol:inbound_tags, empty means all inbounds
    pub async fn modify_user_template(
        &self,
        id: i32,
        body: UserTemplateModify,
    ) -> Result<UserTemplateResponse, ApiError> {
        let url = format!("{}/api/user_template/{}", self.inner.base_url, id);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<UserTemplateResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError(
                "User Template not found".to_string(),
            )),
            StatusCode::CONFLICT => Err(ApiError::ApiResponseError(
                "Template by this name already exists".to_string(),
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

    /// `DELETE /api/user_template/{id}`
    ///
    /// Remove a User Template by its ID
    pub async fn remove_user_template(&self, id: i32) -> Result<String, ApiError> {
        let url = format!("{}/api/user_template/{}", self.inner.base_url, id);
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
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError(
                "User Template not found".to_string(),
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
}
