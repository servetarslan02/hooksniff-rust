use crate::{error::Result, models::*, Configuration};

pub struct RateLimit<'a> { cfg: &'a Configuration }

impl<'a> RateLimit<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self, endpoint_id: String) -> Result<RateLimitOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/endpoint/{endpoint_id}/rate-limit")
            .with_path_param("endpoint_id", endpoint_id)
            .execute(self.cfg).await
    }

    pub async fn set(&self, endpoint_id: String, rate_limit_in: RateLimitIn) -> Result<RateLimitOut> {
        crate::request::Request::new(http1::Method::PUT, "/v1/endpoint/{endpoint_id}/rate-limit")
            .with_path_param("endpoint_id", endpoint_id)
            .with_body_param(rate_limit_in)
            .execute(self.cfg).await
    }

    pub async fn delete(&self, endpoint_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/endpoint/{endpoint_id}/rate-limit")
            .with_path_param("endpoint_id", endpoint_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
