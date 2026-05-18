// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PlaygroundOut {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
