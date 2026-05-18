use serde::{Deserialize, Serialize};
use super::custom_domain_out::CustomDomainOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseCustomDomainOut { pub data: Vec<CustomDomainOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseCustomDomainOut { pub fn new(data: Vec<CustomDomainOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
