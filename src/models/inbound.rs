// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InboundConfigOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The inbound config's ID.
    pub id: String,

    /// The provider name (e.g. "stripe", "github", "shopify").
    pub provider: String,

    /// The signing secret for verifying webhooks.
    pub secret: String,

    /// Optional endpoint ID to forward to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,

    /// Whether this config is enabled.
    pub enabled: bool,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl InboundConfigOut {
    pub fn new(
        created_at: String,
        id: String,
        provider: String,
        secret: String,
        enabled: bool,
        updated_at: String,
    ) -> Self {
        Self {
            created_at,
            id,
            provider,
            secret,
            endpoint_id: None,
            enabled,
            updated_at,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InboundConfigIn {
    /// The provider name (e.g. "stripe", "github", "shopify").
    pub provider: String,

    /// The signing secret for verifying webhooks.
    pub secret: String,

    /// Optional endpoint ID to forward to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,

    /// Whether this config is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl InboundConfigIn {
    pub fn new(provider: String, secret: String) -> Self {
        Self {
            provider,
            secret,
            endpoint_id: None,
            enabled: None,
        }
    }
}
