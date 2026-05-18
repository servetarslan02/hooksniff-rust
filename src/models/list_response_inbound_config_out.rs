// this file is @generated
use serde::{Deserialize, Serialize};

use super::inbound::InboundConfigOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseInboundConfigOut {
    pub data: Vec<InboundConfigOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseInboundConfigOut {
    pub fn new(data: Vec<InboundConfigOut>, done: bool) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
