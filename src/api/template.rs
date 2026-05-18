use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct TemplateListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct TemplateCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Template<'a> { cfg: &'a Configuration }

impl<'a> Template<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<TemplateListOptions>) -> Result<ListResponseTemplateOut> {
        let TemplateListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/templates")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, template_in: TemplateIn, options: Option<TemplateCreateOptions>) -> Result<TemplateOut> {
        let TemplateCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/templates")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(template_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, template_id: String) -> Result<TemplateOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/templates/{template_id}")
            .with_path_param("template_id", template_id)
            .execute(self.cfg).await
    }

    pub async fn update(&self, template_id: String, template_in: TemplateIn) -> Result<TemplateOut> {
        crate::request::Request::new(http1::Method::PUT, "/v1/templates/{template_id}")
            .with_path_param("template_id", template_id)
            .with_body_param(template_in)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, template_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/templates/{template_id}")
            .with_path_param("template_id", template_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
