// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SchemaIn {
    pub name: String,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl SchemaIn {
    pub fn new(name: String, schema: serde_json::Value) -> Self {
        Self { name, schema, description: None }
    }
}
