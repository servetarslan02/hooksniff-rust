use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageRateLimit {
    #[serde(rename = "requests_per_minute")]
    pub requests_per_minute: i32,
}
