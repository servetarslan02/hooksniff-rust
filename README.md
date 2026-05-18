<h1 align="center">
    <a style="text-decoration: none" href="https://hooksniff.vercel.app">
      <img width="120" src="https://avatars.githubusercontent.com/u/80175132?s=200&v=4" />
      <p align="center">HookSniff - Webhook Infrastructure</p>
    </a>
</h1>
<h2 align="center">
  <a href="https://hooksniff.vercel.app">Website</a> | <a href="https://hooksniff.vercel.app/docs">Documentation</a>
</h2>

Rust client library for the [HookSniff](https://hooksniff.vercel.app) webhook delivery platform.

[![Crates.io](https://img.shields.io/crates/v/hooksniff)](https://crates.io/crates/hooksniff)
[![docs.rs](https://docs.rs/hooksniff/badge.svg)](https://docs.rs/hooksniff/)
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

## Features

- **Reliable webhook delivery** — automatic retries with exponential backoff
- **HMAC signatures** — Standard Webhooks compliant (HMAC-SHA256)
- **30+ API resources** — Applications, Endpoints, Messages, Streaming, Integrations, and more
- **Async/await** — built on Tokio + Hyper
- **Type-safe** — full serde deserialization with strongly-typed models

## Installation

```toml
[dependencies]
hooksniff = "1.3"
```

## Quick Start

```rust
use hooksniff::api::HookSniff;

#[tokio::main]
async fn main() {
    let client = HookSniff::new("your-api-token".to_string(), None);

    // List applications
    let apps = client.application().list(None).await.unwrap();
    for app in apps.data {
        println!("App: {} ({})", app.name, app.id);
    }
}
```

## API Resources

| Resource | Method | Description |
|----------|--------|-------------|
| `application()` | CRUD | Manage applications |
| `endpoint()` | CRUD | Manage webhook endpoints |
| `message()` | CRUD | Send and manage messages |
| `message_attempt()` | CRUD | Track delivery attempts |
| `event_type()` | CRUD | Manage event types |
| `environment()` | Export/Import | Organization settings |
| `background_task()` | List/Get | Background task status |
| `connector()` | CRUD | Third-party connectors |
| `integration()` | CRUD | App integrations |
| `inbound()` | CRUD | Inbound webhook configs |
| `streaming()` | CRUD | Real-time streaming |
| `message_poller()` | Poll | Poll-based message consumption |
| `operational_webhook()` | CRUD | Operational webhook endpoints |
| `api_key()` | CRUD | API key management |
| `alert()` | CRUD | Delivery alerts |
| `analytics()` | Aggregate | Usage analytics |
| `billing()` | Get | Billing information |
| `audit_log()` | List | Audit trail |
| `custom_domain()` | CRUD | Custom domains |
| `rate_limit()` | CRUD | Per-endpoint rate limits |
| `routing()` | CRUD | Webhook routing rules |
| `schema()` | CRUD | JSON schema registry |
| `template()` | CRUD | Payload templates |
| `notification()` | List/Read | Notifications |
| `sso()` | Get | SSO configuration |
| `team()` | List | Team members |
| `portal()` | Get | Customer portal |
| `playground()` | Get | API playground |
| `service_token()` | CRUD | Service tokens |
| `statistics()` | Aggregate | Usage statistics |

## Webhook Verification

```rust
use hooksniff::webhooks::Webhook;

let wh = Webhook::new("whsec_...".to_string());
let headers = /* extract headers from request */;
let payload = /* request body */;

let verified = wh.verify(payload, headers)?;
// `verified` contains the parsed event
```

## License

MIT
