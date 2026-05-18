// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SsoConfigOut {
    pub id: String,
    pub provider: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
