use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct AnalyticsAggregateOptions {
    pub idempotency_key: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
}

pub struct Analytics<'a> { cfg: &'a Configuration }

impl<'a> Analytics<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn aggregate(&self, options: Option<AnalyticsAggregateOptions>) -> Result<AnalyticsOut> {
        let AnalyticsAggregateOptions { idempotency_key, since, until } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/stats")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_optional_query_param("since", since)
            .with_optional_query_param("until", until)
            .execute(self.cfg).await
    }
}
