// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TemplateOut {
    pub id: String,
    pub name: String,
    pub payload_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
