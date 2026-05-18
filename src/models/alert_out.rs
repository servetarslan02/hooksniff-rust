// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AlertOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub id: String,
    pub name: String,
    pub alert_type: String,
    pub threshold: f64,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl AlertOut {
    pub fn new(created_at: String, id: String, name: String, alert_type: String, threshold: f64, enabled: bool, updated_at: String) -> Self {
        Self { created_at, id, name, alert_type, threshold, enabled, endpoint_id: None, updated_at }
    }
}
