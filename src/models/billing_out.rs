// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BillingOut {
    pub plan: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_limit: Option<i64>,
}
