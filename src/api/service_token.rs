use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ServiceTokenListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct ServiceTokenCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct ServiceToken<'a> { cfg: &'a Configuration }

impl<'a> ServiceToken<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<ServiceTokenListOptions>) -> Result<ListResponseServiceTokenOut> {
        let ServiceTokenListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/service-tokens")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, service_token_in: ServiceTokenIn, options: Option<ServiceTokenCreateOptions>) -> Result<ServiceTokenOut> {
        let ServiceTokenCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/service-tokens")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(service_token_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, token_id: String) -> Result<ServiceTokenOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/service-tokens/{token_id}")
            .with_path_param("token_id", token_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, token_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/service-tokens/{token_id}")
            .with_path_param("token_id", token_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
