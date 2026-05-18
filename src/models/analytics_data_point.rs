use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AnalyticsDataPoint {
    pub timestamp: String,
    pub count: i64,
}
