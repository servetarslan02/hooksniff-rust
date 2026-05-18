use serde::{Deserialize, Serialize};
use super::audit_log_out::AuditLogOut;
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseAuditLogOut { pub data: Vec<AuditLogOut>, pub done: bool, #[serde(skip_serializing_if = "Option::is_none")] pub iterator: Option<String>, #[serde(rename = "prevIterator")] #[serde(skip_serializing_if = "Option::is_none")] pub prev_iterator: Option<String> }
impl ListResponseAuditLogOut { pub fn new(data: Vec<AuditLogOut>, done: bool) -> Self { Self { data, done, iterator: None, prev_iterator: None } } }
