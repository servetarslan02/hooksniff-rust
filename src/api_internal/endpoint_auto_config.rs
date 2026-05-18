// this file is @generated
use crate::{error::Result, models::*, Configuration};

pub struct EndpointAutoConfig<'a> {
    cfg: &'a Configuration,
}

impl<'a> EndpointAutoConfig<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Update an auto-config endpoint by providing endpoint details.
    pub async fn update(
        &self,
        endpoint_id: String,
        subscribe_in: SubscribeIn,
    ) -> Result<EndpointOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/v1/endpoints/{endpoint_id}/auto-config",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(subscribe_in)
        .execute(self.cfg)
        .await
    }
}
