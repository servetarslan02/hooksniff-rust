// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RoutingOut {
    pub id: String,
    pub name: String,
    pub strategy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ids: Option<Vec<String>>,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
