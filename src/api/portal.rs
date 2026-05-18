use crate::{error::Result, models::*, Configuration};

pub struct Portal<'a> { cfg: &'a Configuration }

impl<'a> Portal<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self, app_id: String) -> Result<PortalOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/app/{app_id}/portal")
            .with_path_param("app_id", app_id)
            .execute(self.cfg).await
    }
}
