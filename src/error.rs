// SPDX-FileCopyrightText: © 2022 HookSniff Authors
// SPDX-License-Identifier: MIT

use std::fmt;

use http_body_util::BodyExt;
use hyper::body::Incoming;

use crate::http1_to_02_status_code;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned from the HookSniff API
#[derive(Debug, Clone)]
pub enum Error {
    /// A generic error
    Generic(String),
    /// Http Error
    Http(HttpErrorContent<crate::models::HttpErrorOut>),
    /// Http Validation Error
    Validation(HttpErrorContent<crate::models::HttpValidationError>),
}

impl Error {
    pub(crate) fn generic(err: impl std::error::Error) -> Self {
        Self::Generic(format!("{err:?}"))
    }

    /// Get the HTTP status code if this is an HTTP error
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Http(e) => Some(e.status.as_u16()),
            Error::Validation(_) => Some(422),
            Error::Generic(_) => None,
        }
    }

    /// Get response headers if available
    pub fn headers(&self) -> Option<&std::collections::HashMap<String, String>> {
        match self {
            Error::Http(e) => e.headers.as_ref(),
            Error::Validation(e) => e.headers.as_ref(),
            Error::Generic(_) => None,
        }
    }

    /// Check if this is a 400 Bad Request error
    pub fn is_bad_request(&self) -> bool { self.status_code() == Some(400) }
    /// Check if this is a 401 Unauthorized error
    pub fn is_unauthorized(&self) -> bool { self.status_code() == Some(401) }
    /// Check if this is a 403 Forbidden error
    pub fn is_forbidden(&self) -> bool { self.status_code() == Some(403) }
    /// Check if this is a 404 Not Found error
    pub fn is_not_found(&self) -> bool { self.status_code() == Some(404) }
    /// Check if this is a 409 Conflict error
    pub fn is_conflict(&self) -> bool { self.status_code() == Some(409) }
    /// Check if this is a 422 Validation error
    pub fn is_validation_error(&self) -> bool { self.status_code() == Some(422) }
    /// Check if this is a 429 Rate Limit error
    pub fn is_rate_limited(&self) -> bool { self.status_code() == Some(429) }
    /// Check if this is a 500 Internal Server Error
    pub fn is_internal_server_error(&self) -> bool { self.status_code() == Some(500) }
    /// Check if this is a 502 Bad Gateway error
    pub fn is_bad_gateway(&self) -> bool { self.status_code() == Some(502) }
    /// Check if this is a 503 Service Unavailable error
    pub fn is_service_unavailable(&self) -> bool { self.status_code() == Some(503) }
    /// Check if this is a 504 Gateway Timeout error
    pub fn is_gateway_timeout(&self) -> bool { self.status_code() == Some(504) }
    /// Check if this is any 5xx server error
    pub fn is_server_error(&self) -> bool { self.status_code().map_or(false, |s| s >= 500) }
    /// Check if this is any 4xx client error
    pub fn is_client_error(&self) -> bool { self.status_code().map_or(false, |s| s >= 400 && s < 500) }

    /// Get the Retry-After value from headers (for 429 errors)
    pub fn retry_after(&self) -> Option<u64> {
        self.headers()
            .and_then(|h| h.get("retry-after"))
            .and_then(|v| v.parse().ok())
    }

    pub(crate) async fn from_response(
        status_code: http1::StatusCode,
        body: Incoming,
        headers: Option<&http1::HeaderMap>,
    ) -> Self {
        let header_map = headers.map(|h| {
            let mut map = std::collections::HashMap::new();
            for (k, v) in h.iter() {
                if let Ok(val) = v.to_str() {
                    map.insert(k.as_str().to_string(), val.to_string());
                }
            }
            map
        });

        match body.collect().await {
            Ok(collected) => {
                let bytes = collected.to_bytes();
                if status_code == http1::StatusCode::UNPROCESSABLE_ENTITY {
                    Self::Validation(HttpErrorContent {
                        status: http02::StatusCode::UNPROCESSABLE_ENTITY,
                        payload: serde_json::from_slice(&bytes).ok(),
                        headers: header_map,
                    })
                } else {
                    Error::Http(HttpErrorContent {
                        status: http1_to_02_status_code(status_code),
                        payload: serde_json::from_slice(&bytes).ok(),
                        headers: header_map,
                    })
                }
            }
            Err(e) => Self::Generic(e.to_string()),
        }
    }
}

// TODO: Remove for v2.0 of the library (very uncommon impl for an error type)
impl From<Error> for String {
    fn from(err: Error) -> Self {
        err.to_string()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Generic(s) => s.fmt(f),
            Error::Http(e) => format!("Http error (status={}) {:?}", e.status, e.payload).fmt(f),
            Error::Validation(e) => format!("Validation error {:?}", e.payload).fmt(f),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct HttpErrorContent<T> {
    pub status: http02::StatusCode,
    pub payload: Option<T>,
    pub headers: Option<std::collections::HashMap<String, String>>,
}

/// Additional error type constructors for common non-HTTP errors
impl Error {
    /// Create a timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Generic(format!("Timeout: {}", message.into()))
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Generic(format!("Network error: {}", message.into()))
    }

    /// Create an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Generic(format!("Authentication error: {}", message.into()))
    }

    /// Check if this is a 408 Request Timeout error
    pub fn is_request_timeout(&self) -> bool { self.status_code() == Some(408) }
    /// Check if this is a 410 Gone error
    pub fn is_gone(&self) -> bool { self.status_code() == Some(410) }
    /// Check if this is a 413 Payload Too Large error
    pub fn is_payload_too_large(&self) -> bool { self.status_code() == Some(413) }
    /// Check if this is a 501 Not Implemented error
    pub fn is_not_implemented(&self) -> bool { self.status_code() == Some(501) }
    /// Check if this is a 507 Insufficient Storage error
    pub fn is_insufficient_storage(&self) -> bool { self.status_code() == Some(507) }
    /// Check if this is a 508 Loop Detected error
    pub fn is_loop_detected(&self) -> bool { self.status_code() == Some(508) }
}
