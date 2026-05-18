use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSubscriptionOut {
    pub id: String,
    #[serde(rename = "streamId")]
    pub stream_id: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
}
