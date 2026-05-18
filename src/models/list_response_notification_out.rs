use serde::{Deserialize, Serialize};
use super::notification_out::NotificationOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseNotificationOut { pub data: Vec<NotificationOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseNotificationOut { pub fn new(data: Vec<NotificationOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
