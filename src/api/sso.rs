use crate::{error::Result, models::*, Configuration};

pub struct Sso<'a> { cfg: &'a Configuration }

impl<'a> Sso<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get_config(&self) -> Result<SsoConfigOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/sso")
            .execute(self.cfg).await
    }
}
