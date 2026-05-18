// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct StreamingStreamListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct StreamingStreamCreateOptions {
    pub idempotency_key: Option<String>,
}

pub struct StreamingStream<'a> {
    cfg: &'a Configuration,
}

impl<'a> StreamingStream<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List of all the organization's streams.
    pub async fn list(
        &self,
        options: Option<StreamingStreamListOptions>,
    ) -> Result<ListResponseStreamOut> {
        let StreamingStreamListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/v1/stream")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Creates a new stream.
    pub async fn create(
        &self,
        stream_in: StreamIn,
        options: Option<StreamingStreamCreateOptions>,
    ) -> Result<StreamOut> {
        let StreamingStreamCreateOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/v1/stream")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(stream_in)
            .execute(self.cfg)
            .await
    }

    /// Get a stream by id or uid.
    pub async fn get(&self, stream_id: String) -> Result<StreamOut> {
        crate::request::Request::new(http1::Method::GET, "/v1/stream/{stream_id}")
            .with_path_param("stream_id", stream_id)
            .execute(self.cfg)
            .await
    }

    /// Update a stream.
    pub async fn update(&self, stream_id: String, stream_in: StreamIn) -> Result<StreamOut> {
        crate::request::Request::new(http1::Method::PUT, "/v1/stream/{stream_id}")
            .with_path_param("stream_id", stream_id)
            .with_body_param(stream_in)
            .execute(self.cfg)
            .await
    }

    /// Delete a stream.
    pub async fn delete(&self, stream_id: String) -> Result<()> {
        crate::request::Request::new(http1::Method::DELETE, "/v1/stream/{stream_id}")
            .with_path_param("stream_id", stream_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Partially update a stream.
    pub async fn patch(&self, stream_id: String, stream_patch: StreamPatch) -> Result<StreamOut> {
        crate::request::Request::new(http1::Method::PATCH, "/v1/stream/{stream_id}")
            .with_path_param("stream_id", stream_id)
            .with_body_param(stream_patch)
            .execute(self.cfg)
            .await
    }
}

use futures::stream::TryStreamExt;

impl<'a> StreamingStream<'a> {
    /// Subscribe to real-time events via SSE on a stream.
    /// Returns a stream of event strings that can be consumed with `futures::Stream`.
    pub async fn subscribe(&self, stream_id: String) -> Result<impl futures::Stream<Item = Result<String>>> {
        let url = format!("{}/v1/stream/{}/subscribe", self.cfg.base_path, stream_id);
        let client = &self.cfg.client;
        let resp = client
            .get(&url)
            .header("Accept", "text/event-stream")
            .header("Authorization", format!("Bearer {}", self.cfg.bearer_token))
            .send()
            .await?;

        let byte_stream = resp.bytes_stream();
        let string_stream = byte_stream.map_err(|e| crate::error::Error::Http(e.to_string()))
            .map_ok(|bytes| String::from_utf8_lossy(&bytes).to_string());

        Ok(string_stream)
    }

    /// Publish an event to a stream.
    pub async fn publish(&self, body: crate::models::PublishEventIn) -> Result<crate::models::PublishEventResponse> {
        crate::request::Request::new(http1::Method::POST, "/v1/stream/events")
            .with_body(body)
            .execute(self.cfg)
            .await
    }

    /// List subscriptions for a stream.
    pub async fn list_subscriptions(&self, stream_id: String) -> Result<Vec<crate::models::StreamSubscriptionOut>> {
        crate::request::Request::new(http1::Method::GET, "/v1/stream/{stream_id}/subscriptions")
            .with_path_param("stream_id", stream_id)
            .execute(self.cfg)
            .await
    }
}
