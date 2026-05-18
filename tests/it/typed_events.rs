// Tests for typed webhook events

use hooksniff::webhook_event::{WebhookEvent, EndpointCreatedEventData, EndpointDisabledEventData,
    MessageAttemptExhaustedEventData, LastAttemptInfo};

#[test]
fn test_parse_endpoint_created_data() {
    let event = WebhookEvent {
        event: "endpoint.created".to_string(),
        data: serde_json::json!({
            "appId": "app_1",
            "endpointId": "ep_1",
            "appUid": "uid_1"
        }),
        timestamp: "2026-05-19".to_string(),
    };

    let data: EndpointCreatedEventData = event.parse_data().unwrap();
    assert_eq!(data.app_id, "app_1");
    assert_eq!(data.endpoint_id, "ep_1");
    assert_eq!(data.app_uid, Some("uid_1".to_string()));
}

#[test]
fn test_parse_endpoint_disabled_data() {
    let event = WebhookEvent {
        event: "endpoint.disabled".to_string(),
        data: serde_json::json!({
            "appId": "app_1",
            "endpointId": "ep_1",
            "failSince": "2026-01",
            "trigger": "repeated-failure"
        }),
        timestamp: "2026-05-19".to_string(),
    };

    let data: EndpointDisabledEventData = event.parse_data().unwrap();
    assert_eq!(data.fail_since, Some("2026-01".to_string()));
    assert_eq!(data.trigger, Some("repeated-failure".to_string()));
}

#[test]
fn test_parse_message_attempt_exhausted_data() {
    let event = WebhookEvent {
        event: "message.attempt.exhausted".to_string(),
        data: serde_json::json!({
            "appId": "app_1",
            "msgId": "msg_1",
            "lastAttempt": {
                "id": "att_1",
                "timestamp": "2026-05-19",
                "responseStatusCode": 500
            }
        }),
        timestamp: "2026-05-19".to_string(),
    };

    let data: MessageAttemptExhaustedEventData = event.parse_data().unwrap();
    assert_eq!(data.msg_id, "msg_1");
    assert_eq!(data.last_attempt.response_status_code, 500);
}

#[test]
fn test_webhook_event_get() {
    let event = WebhookEvent {
        event: "endpoint.created".to_string(),
        data: serde_json::json!({"appId": "app_1"}),
        timestamp: "2026-05-19".to_string(),
    };

    assert_eq!(event.event_type(), "endpoint.created");
    assert_eq!(event.get("appId"), Some(&serde_json::json!("app_1")));
    assert_eq!(event.get("missing"), None);
}

#[test]
fn test_webhook_event_backwards_compat() {
    let event = WebhookEvent {
        event: "test.event".to_string(),
        data: serde_json::json!({"key": "value"}),
        timestamp: "2026-05-19".to_string(),
    };

    // get() should still work with raw JSON
    assert_eq!(event.get("key"), Some(&serde_json::json!("value")));
    assert_eq!(event.event_type(), "test.event");
}

#[test]
fn test_parse_unknown_event_keeps_raw_data() {
    let event = WebhookEvent {
        event: "custom.unknown".to_string(),
        data: serde_json::json!({"x": 1}),
        timestamp: "2026-05-19".to_string(),
    };

    // Should still be able to access raw data
    assert_eq!(event.get("x"), Some(&serde_json::json!(1)));
}

#[test]
fn test_all_endpoint_event_types() {
    let events = vec![
        "endpoint.created",
        "endpoint.updated",
        "endpoint.deleted",
        "endpoint.enabled",
        "endpoint.disabled",
    ];

    for event_type in events {
        let event = WebhookEvent {
            event: event_type.to_string(),
            data: serde_json::json!({"appId": "a", "endpointId": "e"}),
            timestamp: "".to_string(),
        };
        assert_eq!(event.event_type(), event_type);
    }
}

// ═══════════════════════════════════════════════════════════════════
// EDGE CASES
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_empty_data() {
    let event = WebhookEvent {
        event: "endpoint.created".to_string(),
        data: serde_json::json!({}),
        timestamp: "".to_string(),
    };
    let data: EndpointCreatedEventData = event.parse_data().unwrap();
    assert_eq!(data.app_id, "");
    assert_eq!(data.endpoint_id, "");
    assert_eq!(data.app_uid, None);
}

#[test]
fn test_missing_optional_fields() {
    let event = WebhookEvent {
        event: "endpoint.disabled".to_string(),
        data: serde_json::json!({"appId": "a1", "endpointId": "e1"}),
        timestamp: "".to_string(),
    };
    let data: EndpointDisabledEventData = event.parse_data().unwrap();
    assert_eq!(data.fail_since, None);
    assert_eq!(data.trigger, None);
}

#[test]
fn test_extra_fields_ignored() {
    let event = WebhookEvent {
        event: "endpoint.created".to_string(),
        data: serde_json::json!({"appId": "a1", "endpointId": "e1", "extra": "ignored"}),
        timestamp: "".to_string(),
    };
    let data: EndpointCreatedEventData = event.parse_data().unwrap();
    assert_eq!(data.app_id, "a1");
}

#[test]
fn test_nested_json_data() {
    let event = WebhookEvent {
        event: "message.attempt.exhausted".to_string(),
        data: serde_json::json!({
            "appId": "a1",
            "msgId": "m1",
            "lastAttempt": {"id": "att", "timestamp": "t", "responseStatusCode": 500}
        }),
        timestamp: "".to_string(),
    };
    let data: MessageAttemptExhaustedEventData = event.parse_data().unwrap();
    assert_eq!(data.last_attempt.response_status_code, 500);
}

#[test]
fn test_unicode_in_data() {
    let event = WebhookEvent {
        event: "endpoint.created".to_string(),
        data: serde_json::json!({"appId": "ünïcödé", "endpointId": "日本語"}),
        timestamp: "".to_string(),
    };
    let data: EndpointCreatedEventData = event.parse_data().unwrap();
    assert_eq!(data.app_id, "ünïcödé");
    assert_eq!(data.endpoint_id, "日本語");
}
