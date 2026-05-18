// SPDX-FileCopyrightText: © 2026 HookSniff Authors
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

// ─── Event Data Structs ─────────────────────────────────────────────

/// Data payload for endpoint.created events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointCreatedEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for endpoint.updated events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointUpdatedEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for endpoint.deleted events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDeletedEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for endpoint.enabled events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointEnabledEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for endpoint.disabled events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDisabledEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
    #[serde(rename = "failSince")]
    pub fail_since: Option<String>,
    /// "none" | "first-failure" | "repeated-failure"
    pub trigger: Option<String>,
}

/// Info about the last delivery attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastAttemptInfo {
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i32,
}

/// Info about a delivery attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptInfo {
    pub id: String,
    pub timestamp: String,
    #[serde(rename = "responseStatusCode")]
    pub response_status_code: i32,
}

/// Data payload for message.attempt.exhausted events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttemptExhaustedEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    #[serde(rename = "lastAttempt")]
    pub last_attempt: LastAttemptInfo,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for message.attempt.failing events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttemptFailingEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub attempt: AttemptInfo,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

/// Data payload for message.attempt.recovered events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttemptRecoveredEventData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "msgId")]
    pub msg_id: String,
    pub attempt: AttemptInfo,
    #[serde(rename = "appUid")]
    pub app_uid: Option<String>,
}

// ─── Typed Webhook Events ───────────────────────────────────────────

/// A typed webhook event with strongly-typed data payload.
///
/// Use this enum with `match` to get type-safe access to event data:
///
/// ```rust
/// use hooksniff::webhook_event::TypedWebhookEvent;
///
/// match event {
///     TypedWebhookEvent::EndpointCreated(e) => {
///         println!("Endpoint created: {}", e.data.endpoint_id);
///     }
///     TypedWebhookEvent::MessageAttemptExhausted(e) => {
///         println!("Exhausted: status={}", e.data.last_attempt.response_status_code);
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
pub enum TypedWebhookEvent {
    #[serde(rename = "endpoint.created")]
    EndpointCreated(EndpointCreatedEventData),
    #[serde(rename = "endpoint.updated")]
    EndpointUpdated(EndpointUpdatedEventData),
    #[serde(rename = "endpoint.deleted")]
    EndpointDeleted(EndpointDeletedEventData),
    #[serde(rename = "endpoint.enabled")]
    EndpointEnabled(EndpointEnabledEventData),
    #[serde(rename = "endpoint.disabled")]
    EndpointDisabled(EndpointDisabledEventData),
    #[serde(rename = "message.attempt.exhausted")]
    MessageAttemptExhausted(MessageAttemptExhaustedEventData),
    #[serde(rename = "message.attempt.failing")]
    MessageAttemptFailing(MessageAttemptFailingEventData),
    #[serde(rename = "message.attempt.recovered")]
    MessageAttemptRecovered(MessageAttemptRecoveredEventData),
}

/// A parsed webhook event from HookSniff (generic, backward compatible).
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

    /// Try to parse the event data into a [`TypedWebhookEvent`].
    ///
    /// Returns `Some(TypedWebhookEvent)` if the event type is recognized,
    /// or `None` if the event type is unknown or parsing fails.
    pub fn typed(&self) -> Option<TypedWebhookEvent> {
        // Build a wrapper JSON with "event" and "data" for serde tagged deserialization
        let wrapper = serde_json::json!({
            "event": self.event,
            "data": self.data,
        });
        serde_json::from_value(wrapper).ok()
    }

    /// Parse the data payload into a specific typed struct.
    ///
    /// # Example
    /// ```rust
    /// if let Ok(data) = event.parse_data::<EndpointCreatedEventData>() {
    ///     println!("Endpoint: {}", data.endpoint_id);
    /// }
    /// ```
    pub fn parse_data<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data.clone())
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

    /// Verify the webhook signature and parse into a [`TypedWebhookEvent`].
    ///
    /// Returns a strongly-typed event enum variant on success.
    pub fn verify_and_parse_typed<HM: super::HeaderMap>(
        &self,
        payload: &[u8],
        headers: &HM,
    ) -> Result<TypedWebhookEvent, super::WebhookError> {
        self.verify_inner(payload, headers, true)?;

        let event: TypedWebhookEvent =
            serde_json::from_slice(payload).map_err(|_| super::WebhookError::InvalidPayload)?;

        Ok(event)
    }
}
