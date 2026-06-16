use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageWebhooks {
    #[serde(rename = "used")]
    pub used: i32,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i32>,
}
