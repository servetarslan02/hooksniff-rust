use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ApiKeyListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct ApiKeyCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct ApiKey<'a> { cfg: &'a Configuration }

impl<'a> ApiKey<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<ApiKeyListOptions>) -> Result<ListResponseApiKeyOut> {
        let ApiKeyListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/api-key")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, api_key_in: ApiKeyIn, options: Option<ApiKeyCreateOptions>) -> Result<ApiKeyOut> {
        let ApiKeyCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/api-key")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(api_key_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, key_id: String) -> Result<ApiKeyOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/api-key/{key_id}")
            .with_path_param("key_id", key_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, key_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/api-key/{key_id}")
            .with_path_param("key_id", key_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
