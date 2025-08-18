//! # Subscription API Category

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        user::{UserResponse, UserUsagesResponse},
    },
};

impl MarzbanAPIClient {
    /// `GET /sub/{user_token}`
    ///
    /// Provides a subscription link based on the user agent (Clash, V2Ray, etc.).
    ///
    /// ## Note
    ///
    /// Dependent on the user agent, the response will be different.
    ///
    /// For example, if the user agent is Clash, the response will be a Clash subscription link.
    /// If the user agent is a browser, the response will be a web page.
    pub async fn user_subscription(&self, user_token: impl AsRef<str>) -> Result<String, ApiError> {
        let url = format!("{}/sub/{}", self.inner.base_url, user_token.as_ref());
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /sub/{user_token}/info`
    ///
    /// Retrieves detailed information about the user's subscription.
    pub async fn user_subscription_info(
        &self,
        user_token: impl AsRef<str>,
    ) -> Result<UserResponse, ApiError> {
        let url = format!("{}/sub/{}/info", self.inner.base_url, user_token.as_ref());
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
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /sub/{user_token}/usage`
    ///
    /// Fetches the usage statistics for the user within a specified date range.
    ///
    /// ## Parameters
    ///
    /// - `start` - The start date for the range.
    /// - `end` - The end date for the range.
    pub async fn user_get_usage(
        &self,
        user_token: impl AsRef<str>,
        start: Option<impl Into<String>>,
        end: Option<impl Into<String>>,
    ) -> Result<UserUsagesResponse, ApiError> {
        let url = format!("{}/sub/{}/usage", self.inner.base_url, user_token.as_ref());
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
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /sub/{user_token}/{client_type}`
    ///
    /// Provides a subscription link based on the specified client type (e.g., Clash, V2Ray).
    pub async fn user_subscription_with_client_type(
        &self,
        user_token: impl AsRef<str>,
        client_type: ClientTypes,
    ) -> Result<String, ApiError> {
        let url = format!(
            "{}/sub/{}/{}",
            self.inner.base_url,
            user_token.as_ref(),
            client_type
        );
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response.text().await.map_err(ApiError::NetworkError),
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

pub enum ClientTypes {
    SingBox,
    ClashMeta,
    Clash,
    Outline,
    V2Ray,
    V2RayJSON,
}

impl std::fmt::Display for ClientTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientTypes::SingBox => write!(f, "sing-box"),
            ClientTypes::ClashMeta => write!(f, "clash-meta"),
            ClientTypes::Clash => write!(f, "clash"),
            ClientTypes::Outline => write!(f, "outline"),
            ClientTypes::V2Ray => write!(f, "v2ray"),
            ClientTypes::V2RayJSON => write!(f, "v2ray-json"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_client_types() {
        assert_eq!(ClientTypes::Clash.to_string(), "clash")
    }
}
