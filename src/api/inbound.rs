// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct InboundListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct InboundCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct Inbound<'a> {
    cfg: &'a Configuration,
}

impl<'a> Inbound<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List inbound webhook configs.
    pub async fn list(
        &self,
        options: Option<InboundListOptions>,
    ) -> Result<ListResponseInboundConfigOut> {
        let InboundListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/v1/inbound/configs")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Create an inbound webhook config.
    pub async fn create(
        &self,
        inbound_config_in: InboundConfigIn,
        options: Option<InboundCreateOptions>,
    ) -> Result<InboundConfigOut> {
        let InboundCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/v1/inbound/configs")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(inbound_config_in)
            .execute(self.cfg)
            .await
    }

    /// Get an inbound webhook config.
    pub async fn get(&self, inbound_id: String) -> Result<InboundConfigOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/v1/inbound/configs/{inbound_id}",
        )
        .with_path_param("inbound_id", inbound_id)
        .execute(self.cfg)
        .await
    }

    /// Update an inbound webhook config.
    pub async fn update(
        &self,
        inbound_id: String,
        inbound_config_in: InboundConfigIn,
    ) -> Result<InboundConfigOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/v1/inbound/configs/{inbound_id}",
        )
        .with_path_param("inbound_id", inbound_id)
        .with_body_param(inbound_config_in)
        .execute(self.cfg)
        .await
    }

    /// Delete an inbound webhook config.
    pub async fn delete(&self, inbound_id: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/v1/inbound/configs/{inbound_id}",
        )
        .with_path_param("inbound_id", inbound_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
