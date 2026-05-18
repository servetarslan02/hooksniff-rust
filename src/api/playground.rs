use crate::{error::Result, models::*, Configuration};

pub struct Playground<'a> { cfg: &'a Configuration }

impl<'a> Playground<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn get(&self, app_id: String) -> Result<PlaygroundOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/app/{app_id}/playground")
            .with_path_param("app_id", app_id)
            .execute(self.cfg).await
    }
}
