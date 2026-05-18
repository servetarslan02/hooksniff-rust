// SPDX-FileCopyrightText: © 2026 HookSniff Authors
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// A parsed webhook event from HookSniff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Event type name (e.g., "endpoint.created")
    pub event: String,
    /// Event payload data
    pub data: serde_json::Value,
    /// ISO 8601 timestamp string
    pub timestamp: String,
}

impl WebhookEvent {
    /// Get the event type name.
    pub fn event_type(&self) -> &str {
        &self.event
    }

    /// Get a value from the data payload by key.
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

impl super::Webhook {
    /// Verify the webhook signature and parse the payload into a [`WebhookEvent`].
    ///
    /// Returns the parsed `WebhookEvent` on success, or a [`WebhookError`] if
    /// the signature is invalid, the timestamp is outside tolerance, or the
    /// payload cannot be parsed.
    pub fn verify_and_parse<HM: super::HeaderMap>(
        &self,
        payload: &[u8],
        headers: &HM,
    ) -> Result<WebhookEvent, super::WebhookError> {
        self.verify_inner(payload, headers, true)?;

        let event: WebhookEvent =
            serde_json::from_slice(payload).map_err(|_| super::WebhookError::InvalidPayload)?;

        Ok(event)
    }

    /// Verify the webhook signature (ignoring timestamp) and parse the payload
    /// into a [`WebhookEvent`].
    ///
    /// WARNING: This function does not check the signature's timestamp.
    /// We recommend using [`verify_and_parse`](Self::verify_and_parse) instead.
    pub fn verify_and_parse_ignoring_timestamp<HM: super::HeaderMap>(
        &self,
        payload: &[u8],
        headers: &HM,
    ) -> Result<WebhookEvent, super::WebhookError> {
        self.verify_inner(payload, headers, false)?;

        let event: WebhookEvent =
            serde_json::from_slice(payload).map_err(|_| super::WebhookError::InvalidPayload)?;

        Ok(event)
    }
}
