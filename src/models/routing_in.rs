// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RoutingIn {
    pub name: String,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl RoutingIn {
    pub fn new(name: String, strategy: String) -> Self {
        Self { name, strategy, endpoint_ids: None, enabled: None }
    }
}
