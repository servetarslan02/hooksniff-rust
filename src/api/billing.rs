use crate::{error::Result, models::*, Configuration};

pub struct Billing<'a> { cfg: &'a Configuration }

impl<'a> Billing<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self) -> Result<BillingOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/billing")
            .execute(self.cfg).await
    }
}
