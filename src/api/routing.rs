use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct RoutingListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct RoutingCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Routing<'a> { cfg: &'a Configuration }

impl<'a> Routing<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<RoutingListOptions>) -> Result<ListResponseRoutingOut> {
        let RoutingListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/routing")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, routing_in: RoutingIn, options: Option<RoutingCreateOptions>) -> Result<RoutingOut> {
        let RoutingCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/routing")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(routing_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, routing_id: String) -> Result<RoutingOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/routing/{routing_id}")
            .with_path_param("routing_id", routing_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, routing_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/routing/{routing_id}")
            .with_path_param("routing_id", routing_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
