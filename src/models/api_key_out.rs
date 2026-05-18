// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApiKeyOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub scopes: Vec<String>,
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl ApiKeyOut {
    pub fn new(created_at: String, id: String, name: String, scopes: Vec<String>) -> Self {
        Self { created_at, id, name, note: None, scopes, expires_at: None }
    }
}
