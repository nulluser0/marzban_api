//! # Node API Category

use reqwest::StatusCode;

use crate::{
    client::MarzbanAPIClient,
    error::ApiError,
    models::{
        errors::HTTPValidationError,
        node::{NodeCreate, NodeModify, NodeResponse, NodeSettings, NodesUsageResponse},
    },
};

impl MarzbanAPIClient {
    /// `GET /api/node/settings`
    ///
    /// Retrieve the current node settings, including TLS certificate.
    pub async fn get_node_settings(&self) -> Result<NodeSettings, ApiError> {
        let url = format!("{}/api/node/settings", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeSettings>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/node`
    ///
    /// Add a new node to the database and optionally add it as a host.
    pub async fn add_node(&self, body: NodeCreate) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::POST, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
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

    /// `GET /api/node/{node_id}`
    ///
    /// Retrieve details of a specific node by its ID.
    pub async fn get_node(&self, node_id: i32) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node/{}", self.inner.base_url, node_id);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `PUT /api/node/{node_id}`
    ///
    /// Update a node's details. Only accessible to sudo admins.
    pub async fn modify_node(
        &self,
        node_id: i32,
        body: NodeModify,
    ) -> Result<NodeResponse, ApiError> {
        let url = format!("{}/api/node/{}", self.inner.base_url, node_id);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::PUT, url.clone())
                    .json(&body)
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<NodeResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `DELETE /api/node/{node_id}`
    ///
    /// Delete a node and remove it from xray in the background.
    pub async fn remove_node(&self, node_id: i32) -> Result<String, ApiError> {
        let url = format!("{}/api/node/{}", self.inner.base_url, node_id);
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
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/nodes`
    ///
    /// Retrieve a list of all nodes. Accessible only to sudo admins.
    pub async fn get_nodes(&self) -> Result<Vec<NodeResponse>, ApiError> {
        let url = format!("{}/api/nodes", self.inner.base_url);
        let response = self
            .send_with_auth_retry(|| async {
                self.prepare_request(reqwest::Method::GET, url.clone())
            })
            .await?;

        match response.status() {
            StatusCode::OK => response
                .json::<Vec<NodeResponse>>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `POST /api/node/{node_id}/reconnect`
    ///
    /// Trigger a reconnection for the specified node. Only accessible to sudo admins.
    pub async fn reconnect_node(&self, node_id: i32) -> Result<String, ApiError> {
        let url = format!("{}/api/node/{}/reconnect", self.inner.base_url, node_id);
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
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
            StatusCode::UNPROCESSABLE_ENTITY => {
                let error_response = response.json::<HTTPValidationError>().await?;
                Err(ApiError::ApiResponseError(format!(
                    "Validation Error: {error_response:?}"
                )))
            }
            _ => Err(ApiError::UnexpectedResponse),
        }
    }

    /// `GET /api/nodes/usage`
    ///
    /// Retrieve usage statistics for nodes within a specified date range.
    ///
    /// ## Parameters
    ///
    /// - `start` - The start date for the range.
    /// - `end` - The end date for the range.
    pub async fn get_nodes_usage(
        &self,
        start: Option<impl Into<String>>,
        end: Option<impl Into<String>>,
    ) -> Result<NodesUsageResponse, ApiError> {
        let url = format!("{}/api/nodes/usage", self.inner.base_url);
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
                .json::<NodesUsageResponse>()
                .await
                .map_err(ApiError::NetworkError),
            StatusCode::FORBIDDEN => {
                Err(ApiError::ApiResponseError("You're not allowed".to_string()))
            }
            StatusCode::NOT_FOUND => Err(ApiError::ApiResponseError("Node not found".to_string())),
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
