use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishEventResponse {
    pub id: String,
    pub channel: String,
    pub event: String,
    pub data: serde_json::Value,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}
