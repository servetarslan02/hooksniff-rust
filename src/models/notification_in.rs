// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NotificationIn {
    pub notification_type: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
}

impl NotificationIn {
    pub fn new(notification_type: String, message: String) -> Self {
        Self { notification_type, message, read: None }
    }
}
