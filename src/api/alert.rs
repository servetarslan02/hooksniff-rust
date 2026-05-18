use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct AlertListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct AlertCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Alert<'a> { cfg: &'a Configuration }

impl<'a> Alert<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<AlertListOptions>) -> Result<ListResponseAlertOut> {
        let AlertListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/alerts")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, alert_in: AlertIn, options: Option<AlertCreateOptions>) -> Result<AlertOut> {
        let AlertCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/alerts")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(alert_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, alert_id: String) -> Result<AlertOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/alerts/{alert_id}")
            .with_path_param("alert_id", alert_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, alert_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/alerts/{alert_id}")
            .with_path_param("alert_id", alert_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
