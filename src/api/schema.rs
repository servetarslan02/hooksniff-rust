use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct SchemaListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct SchemaCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Schema<'a> { cfg: &'a Configuration }

impl<'a> Schema<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<SchemaListOptions>) -> Result<ListResponseSchemaOut> {
        let SchemaListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/schema")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, schema_in: SchemaIn, options: Option<SchemaCreateOptions>) -> Result<SchemaOut> {
        let SchemaCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/schema")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(schema_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, schema_id: String) -> Result<SchemaOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/schema/{schema_id}")
            .with_path_param("schema_id", schema_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, schema_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/schema/{schema_id}")
            .with_path_param("schema_id", schema_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
