use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageResponse {
    #[serde(rename = "plan")]
    pub plan: String,
    #[serde(rename = "payment_provider", skip_serializing_if = "Option::is_none")]
    pub payment_provider: Option<String>,
    #[serde(rename = "webhooks")]
    pub webhooks: models::UsageWebhooks,
    #[serde(rename = "endpoints")]
    pub endpoints: models::UsageEndpoints,
    #[serde(rename = "rate_limit", skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<Box<models::UsageRateLimit>>,
    #[serde(rename = "retention_days", skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[serde(rename = "data_age_days", skip_serializing_if = "Option::is_none")]
    pub data_age_days: Option<i32>,
    #[serde(rename = "data_expires_in_days", skip_serializing_if = "Option::is_none")]
    pub data_expires_in_days: Option<i32>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<Box<models::UsagePeriod>>,
}

impl UsageResponse {
    pub fn new(plan: String, webhooks: models::UsageWebhooks, endpoints: models::UsageEndpoints) -> UsageResponse {
        UsageResponse {
            plan,
            payment_provider: None,
            webhooks,
            endpoints,
            rate_limit: None,
            retention_days: None,
            data_age_days: None,
            data_expires_in_days: None,
            period: None,
        }
    }
}
