use crate::{error::Result, models::*, Configuration};

pub struct Portal<'a> { cfg: &'a Configuration }

impl<'a> Portal<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self) -> Result<PortalOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/portal")
            .execute(self.cfg).await
    }
}
