use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishEventIn {
    pub channel: String,
    pub event: String,
    pub data: serde_json::Value,
}
