// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RateLimitOut {
    pub id: String,
    pub endpoint_id: String,
    pub rate: i32,
    pub window_seconds: i32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
