// SPDX-FileCopyrightText: © 2026 HookSniff Authors
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Response metadata from the last API request.
///
/// Access via `client.last_response()` after any API call.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseMetadata {
    /// HTTP status code
    pub status_code: u16,
    /// x-request-id header (for debugging with HookSniff support)
    pub request_id: Option<String>,
    /// x-ratelimit-remaining header
    pub rate_limit_remaining: Option<i64>,
    /// x-ratelimit-reset header (Unix timestamp)
    pub rate_limit_reset: Option<i64>,
    /// All response headers as key-value pairs
    pub headers: std::collections::HashMap<String, String>,
}

impl ResponseMetadata {
    /// Create from HTTP response parts (status code + headers).
    pub fn from_parts(status_code: u16, headers: &http1::HeaderMap) -> Self {
        let mut hm = std::collections::HashMap::new();
        for (k, v) in headers.iter() {
            if let Ok(val) = v.to_str() {
                hm.insert(k.as_str().to_string(), val.to_string());
            }
        }

        Self {
            status_code,
            request_id: hm.get("x-request-id").cloned(),
            rate_limit_remaining: hm.get("x-ratelimit-remaining").and_then(|v| v.parse().ok()),
            rate_limit_reset: hm.get("x-ratelimit-reset").and_then(|v| v.parse().ok()),
            headers: hm,
        }
    }
}
