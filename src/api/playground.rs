use crate::{error::Result, models::*, Configuration};

pub struct Playground<'a> { cfg: &'a Configuration }

impl<'a> Playground<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self) -> Result<PlaygroundOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/playground")
            .execute(self.cfg).await
    }
}
