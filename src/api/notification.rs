use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct NotificationListOptions {
    pub limit: Option<i32>,
    pub iterator: Option<String>,
    pub order: Option<Ordering>,
}

pub struct Notification<'a> { cfg: &'a Configuration }

impl<'a> Notification<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self { Self { cfg } }

    pub async fn list(&self, options: Option<NotificationListOptions>) -> Result<ListResponseNotificationOut> {
        let NotificationListOptions { limit, iterator, order } = options.unwrap_or_default();
        crate::request::Request::new(http1::Method::GET, "/v1/notification")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg).await
    }

    pub async fn mark_read(&self, notification_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::POST, "/v1/notification/{notification_id}/read")
            .with_path_param("notification_id", notification_id)
            .returns_nothing()
            .execute(self.cfg).await
    }
}
