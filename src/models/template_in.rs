// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TemplateIn {
    pub name: String,
    pub payload_template: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl TemplateIn {
    pub fn new(name: String, payload_template: String) -> Self {
        Self { name, payload_template, description: None, enabled: None }
    }
}
