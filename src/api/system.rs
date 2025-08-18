//! # System API Category

use std::collections::HashMap;

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        proxy::{ProxyHost, ProxyInbound, ProxyTypes},
        system::SystemStats,
    },
};

impl MarzbanAPIClient {
    /// `GET /api/system`
    ///
    /// Fetch system stats including memory, CPU, and user metrics.
    pub async fn get_system_stats(&self) -> Result<SystemStats, ApiError> {
        let url = format!("{}/api/system", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<SystemStats>()
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

    /// `GET /api/inbounds`
    ///
    /// Retrieve inbound configurations grouped by protocol.
    pub async fn get_inbounds(&self) -> Result<HashMap<ProxyTypes, Vec<ProxyInbound>>, ApiError> {
        let url = format!("{}/api/inbounds", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<ProxyTypes, Vec<ProxyInbound>>>()
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

    /// `GET /api/inbounds`
    ///
    /// Get a list of proxy hosts grouped by inbound tag.
    pub async fn get_hosts(&self) -> Result<HashMap<ProxyTypes, Vec<ProxyHost>>, ApiError> {
        let url = format!("{}/api/hosts", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<ProxyTypes, Vec<ProxyHost>>>()
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

    /// `PUT /api/hosts`
    ///
    /// Modify proxy hosts and update the configuration.
    pub async fn modify_hosts(
        &self,
        body: impl Into<HashMap<String, Vec<ProxyHost>>>,
    ) -> Result<HashMap<String, Vec<ProxyHost>>, ApiError> {
        let url = format!("{}/api/hosts", self.inner.base_url);
        let body = body.into();
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<HashMap<String, Vec<ProxyHost>>>()
                .await
                .map_err(ApiError::NetworkError),
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
