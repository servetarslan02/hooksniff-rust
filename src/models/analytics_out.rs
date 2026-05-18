use serde::{Deserialize, Serialize};

use super::analytics_data_point::AnalyticsDataPoint;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnalyticsOut {
    pub total_messages: i64,
    pub successful_deliveries: i64,
    pub failed_deliveries: i64,
    pub average_latency_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_points: Option<Vec<AnalyticsDataPoint>>,
}
