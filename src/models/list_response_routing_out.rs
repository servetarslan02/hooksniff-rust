use serde::{Deserialize, Serialize};
use super::routing_out::RoutingOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseRoutingOut { pub data: Vec<RoutingOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseRoutingOut { pub fn new(data: Vec<RoutingOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
