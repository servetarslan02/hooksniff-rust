// this file is @generated
use super::EndpointAutoConfig;
use crate::{error::Result, models::*, Configuration};

pub struct Endpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> Endpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn auto_config(&self) -> EndpointAutoConfig<'a> {
        EndpointAutoConfig::new(self.cfg)
    }

    /// This operation was renamed to `set-transformation`.
    #[deprecated]
    pub async fn transformation_partial_update(
        &self,
        endpoint_id: String,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PATCH,
            "/v1/endpoints/{endpoint_id}/transformation",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(endpoint_transformation_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
