use serde::{Deserialize, Serialize};
use super::alert_out::AlertOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseAlertOut { pub data: Vec<AlertOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseAlertOut { pub fn new(data: Vec<AlertOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
