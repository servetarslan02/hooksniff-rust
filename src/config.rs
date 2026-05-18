// SPDX-FileCopyrightText: © 2026 HookSniff Authors
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Configuration options for the HookSniff client.
#[derive(Debug, Clone)]
pub struct HookSniffConfig {
    /// Base URL of the HookSniff API.
    pub server_url: String,
    /// Request timeout in seconds.
    pub timeout: u64,
    /// Enable debug logging.
    pub debug: bool,
    /// Custom headers to include in every request.
    pub headers: HashMap<String, String>,
    /// Retry schedule (delays in seconds between retries).
    pub retry_schedule: Vec<u64>,
}

impl Default for HookSniffConfig {
    fn default() -> Self {
        Self {
            server_url: "https://hooksniff-api-1046140057667.europe-west1.run.app".to_string(),
            timeout: 30,
            debug: false,
            headers: HashMap::new(),
            retry_schedule: vec![1, 2, 4],
        }
    }
}

impl HookSniffConfig {
    /// Create a new config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the server URL.
    pub fn server_url(mut self, url: impl Into<String>) -> Self {
        self.server_url = url.into();
        self
    }

    /// Set the request timeout in seconds.
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }

    /// Enable or disable debug logging.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Add a custom header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Set the retry schedule (delays in seconds).
    pub fn retry_schedule(mut self, schedule: Vec<u64>) -> Self {
        self.retry_schedule = schedule;
        self
    }
}
