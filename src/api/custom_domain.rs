use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct CustomDomainListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct CustomDomainCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct CustomDomain<'a> { cfg: &'a Configuration }

impl<'a> CustomDomain<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<CustomDomainListOptions>) -> Result<ListResponseCustomDomainOut> {
        let CustomDomainListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/custom-domains")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn create(&self, custom_domain_in: CustomDomainIn, options: Option<CustomDomainCreateOptions>) -> Result<CustomDomainOut> {
        let CustomDomainCreateOptions { idempotency_key } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::POST, "/v1/custom-domains")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(custom_domain_in)
            .execute(self.cfg).await
    }

    pub async fn get(&self, domain_id: String) -> Result<CustomDomainOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/custom-domains/{domain_id}")
            .with_path_param("domain_id", domain_id)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, domain_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/custom-domains/{domain_id}")
            .with_path_param("domain_id", domain_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
