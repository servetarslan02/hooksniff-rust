// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TeamOut {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TeamMemberOut>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TeamMemberOut {
    pub id: String,
    pub email: String,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl TeamOut {
    pub fn new(id: String, name: String, created_at: String) -> Self {
        Self { id, name, members: None, created_at }
    }
}
