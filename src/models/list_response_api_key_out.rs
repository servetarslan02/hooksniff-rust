use serde::{Deserialize, Serialize};
use super::api_key_out::ApiKeyOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseApiKeyOut { pub data: Vec<ApiKeyOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseApiKeyOut { pub fn new(data: Vec<ApiKeyOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
