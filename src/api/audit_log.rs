use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct AuditLogListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
    pub since: Option<String>,
    pub until: Option<String>,
}

pub struct AuditLog<'a> { cfg: &'a Configuration }

impl<'a> AuditLog<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<AuditLogListOptions>) -> Result<ListResponseAuditLogOut> {
        let AuditLogListOptions { limit, iterator, order, since, until } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/audit-log")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .with_optional_query_param("since", since)
            .with_optional_query_param("until", until)
            .execute(self.cfg).await
    }
}
