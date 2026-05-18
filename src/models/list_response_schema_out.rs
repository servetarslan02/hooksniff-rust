use serde::{Deserialize, Serialize};
use super::schema_out::SchemaOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseSchemaOut { pub data: Vec<SchemaOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseSchemaOut { pub fn new(data: Vec<SchemaOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
