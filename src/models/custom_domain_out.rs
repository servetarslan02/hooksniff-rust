// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomDomainOut {
    pub id: String,
    pub domain: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
