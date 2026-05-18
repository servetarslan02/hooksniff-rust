use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct TeamListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

pub struct Team<'a> { cfg: &'a Configuration }

impl<'a> Team<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<TeamListOptions>) -> Result<Vec<TeamOut>> {
        let TeamListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/teams")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }
}
