// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationOut {
    pub id: String,
    pub notification_type: String,
    pub message: String,
    pub read: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
