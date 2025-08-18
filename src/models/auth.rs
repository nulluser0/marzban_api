use serde::{Deserialize, Serialize};

use crate::models::base::default_empty_string;

// Orignally named: 'Body_admin_token_api_admin_token_post' in the openapi.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyAdminTokenApiAdminTokenPost {
    pub grant_type: Option<String>,
    pub username: String,
    pub password: String,
    #[serde(default = "default_empty_string")]
    pub scope: String,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}
