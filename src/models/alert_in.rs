// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AlertIn {
    pub name: String,
    pub alert_type: String,
    pub threshold: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
}

impl AlertIn {
    pub fn new(name: String, alert_type: String, threshold: f64) -> Self {
        Self { name, alert_type, threshold, enabled: None, endpoint_id: None }
    }
}
