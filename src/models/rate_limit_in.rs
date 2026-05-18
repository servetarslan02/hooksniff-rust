// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RateLimitIn {
    pub endpoint_id: String,
    pub rate: i32,
    pub window_seconds: i32,
}

impl RateLimitIn {
    pub fn new(endpoint_id: String, rate: i32, window_seconds: i32) -> Self {
        Self { endpoint_id, rate, window_seconds }
    }
}
