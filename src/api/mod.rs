// this file is @generated
#![warn(unreachable_pub)]

mod client;
mod deprecated;

pub use self::client::{HookSniff, HookSniffOptions};
pub use crate::models::*;

mod alert;
mod analytics;
mod api_key;
mod authentication;
mod audit_log;
mod background_task;
mod billing;
mod connector;
mod custom_domain;
mod endpoint;
mod environment;
mod event_type;
pub mod inbound;
mod ingest;
mod ingest_endpoint;
mod ingest_source;
mod integration;
mod message;
mod message_attempt;
mod message_poller;
mod notification;
mod operational_webhook;
mod operational_webhook_endpoint;
mod playground;
mod portal;
mod rate_limit;
mod routing;
mod schema;
mod service_token;
mod sso;
mod statistics;
mod streaming;
mod streaming_event_type;
mod streaming_events;
mod streaming_sink;
mod streaming_stream;
mod team;
mod template;

#[cfg(feature = "hooksniff_beta")]
pub use self::message::{V1MessageEventsParams, V1MessageEventsSubscriptionParams};
pub use self::{
    alert::{Alert, AlertCreateOptions, AlertListOptions},
    analytics::{Analytics, AnalyticsAggregateOptions},
    api_key::{ApiKey, ApiKeyCreateOptions, ApiKeyListOptions},
    authentication::{
        Authentication, 
        AuthenticationLogoutOptions, AuthenticationRotateStreamPollerTokenOptions,
        AuthenticationStreamExpireAllOptions, AuthenticationStreamLogoutOptions,
        AuthenticationStreamPortalAccessOptions,
    },
    audit_log::{AuditLog, AuditLogListOptions},
    background_task::{BackgroundTask, BackgroundTaskListOptions},
    billing::Billing,
    connector::{Connector, ConnectorCreateOptions, ConnectorListOptions},
    custom_domain::{CustomDomain, CustomDomainCreateOptions, CustomDomainListOptions},
    deprecated::*,
    endpoint::{
        Endpoint, EndpointBulkReplayOptions, EndpointCreateOptions, EndpointGetStatsOptions,
        EndpointListOptions, EndpointRecoverOptions, EndpointReplayMissingOptions,
        EndpointRotateSecretOptions, EndpointSendExampleOptions,
    },
    environment::{Environment, EnvironmentExportOptions, EnvironmentImportOptions},
    event_type::{
        EventType, EventTypeCreateOptions, EventTypeDeleteOptions, EventTypeImportOpenapiOptions,
        EventTypeListOptions,
    },
    inbound::{Inbound, InboundCreateOptions, InboundListOptions},
    ingest::{Ingest, IngestDashboardOptions},
    ingest_endpoint::{
        IngestEndpoint, IngestEndpointCreateOptions, IngestEndpointListOptions,
        IngestEndpointRotateSecretOptions,
    },
    ingest_source::{
        IngestSource, IngestSourceCreateOptions, IngestSourceListOptions,
        IngestSourceRotateTokenOptions,
    },
    integration::{
        Integration, IntegrationCreateOptions, IntegrationListOptions, IntegrationRotateKeyOptions,
    },
    message::{
        Message, MessageCreateOptions, MessageExpungeAllContentsOptions, MessageGetOptions,
        MessageListOptions, MessagePrecheckOptions,
    },
    message_attempt::{
        MessageAttempt, MessageAttemptGetOptions, MessageAttemptListAttemptedDestinationsOptions,
        MessageAttemptListAttemptedMessagesOptions, MessageAttemptListByEndpointOptions,
        MessageAttemptListByMsgOptions, MessageAttemptResendOptions,
    },
    message_poller::{
        MessagePoller, MessagePollerConsumerPollOptions, MessagePollerConsumerSeekOptions,
        MessagePollerPollOptions,
    },
    notification::{Notification, NotificationListOptions},
    operational_webhook::OperationalWebhook,
    operational_webhook_endpoint::{
        OperationalWebhookEndpoint, OperationalWebhookEndpointCreateOptions,
        OperationalWebhookEndpointListOptions, OperationalWebhookEndpointRotateSecretOptions,
    },
    playground::Playground,
    portal::Portal,
    rate_limit::RateLimit,
    routing::{Routing, RoutingCreateOptions, RoutingListOptions},
    schema::{Schema, SchemaCreateOptions, SchemaListOptions},
    service_token::{ServiceToken, ServiceTokenCreateOptions, ServiceTokenListOptions},
    sso::Sso,
    statistics::{Statistics, StatisticsAggregateAppStatsOptions},
    streaming::Streaming,
    streaming_event_type::{
        StreamingEventType, StreamingEventTypeCreateOptions, StreamingEventTypeDeleteOptions,
        StreamingEventTypeListOptions,
    },
    streaming_events::{StreamingEvents, StreamingEventsCreateOptions, StreamingEventsGetOptions},
    streaming_sink::{
        StreamingSink, StreamingSinkCreateOptions, StreamingSinkListOptions,
        StreamingSinkRotateSecretOptions,
    },
    streaming_stream::{StreamingStream, StreamingStreamCreateOptions, StreamingStreamListOptions},
    team::Team,
    template::{Template, TemplateCreateOptions, TemplateListOptions},
};

impl HookSniff {
    pub fn alert(&self) -> Alert<'_> {
        Alert::new(&self.cfg)
    }

    pub fn analytics(&self) -> Analytics<'_> {
        Analytics::new(&self.cfg)
    }

    pub fn api_key(&self) -> ApiKey<'_> {
        ApiKey::new(&self.cfg)
    }


    pub fn authentication(&self) -> Authentication<'_> {
        Authentication::new(&self.cfg)
    }

    pub fn audit_log(&self) -> AuditLog<'_> {
        AuditLog::new(&self.cfg)
    }

    pub fn background_task(&self) -> BackgroundTask<'_> {
        BackgroundTask::new(&self.cfg)
    }

    pub fn billing(&self) -> Billing<'_> {
        Billing::new(&self.cfg)
    }

    pub fn connector(&self) -> Connector<'_> {
        Connector::new(&self.cfg)
    }

    pub fn custom_domain(&self) -> CustomDomain<'_> {
        CustomDomain::new(&self.cfg)
    }

    pub fn endpoint(&self) -> Endpoint<'_> {
        Endpoint::new(&self.cfg)
    }

    pub fn environment(&self) -> Environment<'_> {
        Environment::new(&self.cfg)
    }

    pub fn event_type(&self) -> EventType<'_> {
        EventType::new(&self.cfg)
    }

    pub fn inbound(&self) -> Inbound<'_> {
        Inbound::new(&self.cfg)
    }

    pub fn ingest(&self) -> Ingest<'_> {
        Ingest::new(&self.cfg)
    }

    pub fn integration(&self) -> Integration<'_> {
        Integration::new(&self.cfg)
    }

    pub fn message(&self) -> Message<'_> {
        Message::new(&self.cfg)
    }

    pub fn message_attempt(&self) -> MessageAttempt<'_> {
        MessageAttempt::new(&self.cfg)
    }

    pub fn message_poller(&self) -> MessagePoller<'_> {
        MessagePoller::new(&self.cfg)
    }

    pub fn notification(&self) -> Notification<'_> {
        Notification::new(&self.cfg)
    }

    pub fn operational_webhook(&self) -> OperationalWebhook<'_> {
        OperationalWebhook::new(&self.cfg)
    }

    pub fn playground(&self) -> Playground<'_> {
        Playground::new(&self.cfg)
    }

    pub fn portal(&self) -> Portal<'_> {
        Portal::new(&self.cfg)
    }

    pub fn rate_limit(&self) -> RateLimit<'_> {
        RateLimit::new(&self.cfg)
    }

    pub fn routing(&self) -> Routing<'_> {
        Routing::new(&self.cfg)
    }

    pub fn schema(&self) -> Schema<'_> {
        Schema::new(&self.cfg)
    }

    pub fn service_token(&self) -> ServiceToken<'_> {
        ServiceToken::new(&self.cfg)
    }

    pub fn sso(&self) -> Sso<'_> {
        Sso::new(&self.cfg)
    }

    pub fn statistics(&self) -> Statistics<'_> {
        Statistics::new(&self.cfg)
    }

    pub fn streaming(&self) -> Streaming<'_> {
        Streaming::new(&self.cfg)
    }

    pub fn team(&self) -> Team<'_> {
        Team::new(&self.cfg)
    }

    pub fn template(&self) -> Template<'_> {
        Template::new(&self.cfg)
    }

    #[deprecated = "Use .operational_webhook().endpoint() instead"]
    pub fn operational_webhook_endpoint(&self) -> OperationalWebhookEndpoint<'_> {
        OperationalWebhookEndpoint::new(&self.cfg)
    }
}
