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

// ═══════════════════════════════════════════════════════════════════
// SIGNATURE TESTS
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_sign_deterministic() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let sig1 = wh.sign("msg_1", 1700000000, b"payload").unwrap();
    let sig2 = wh.sign("msg_1", 1700000000, b"payload").unwrap();
    assert_eq!(sig1, sig2);
}

#[test]
fn test_sign_different_payloads() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let sig1 = wh.sign("msg_1", 1700000000, b"p1").unwrap();
    let sig2 = wh.sign("msg_1", 1700000000, b"p2").unwrap();
    assert_ne!(sig1, sig2);
}

#[test]
fn test_sign_different_secrets() {
    let wh1 = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let wh2 = super::Webhook::new("whsec_b3RoZXI=").unwrap();
    let sig1 = wh1.sign("msg_1", 1700000000, b"p").unwrap();
    let sig2 = wh2.sign("msg_1", 1700000000, b"p").unwrap();
    assert_ne!(sig1, sig2);
}

#[test]
fn test_sign_format() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let sig = wh.sign("msg_1", 1700000000, b"p").unwrap();
    assert!(sig.starts_with("v1,"));
}

#[test]
fn test_sign_empty_payload() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let sig = wh.sign("msg_1", 1700000000, b"").unwrap();
    assert!(sig.starts_with("v1,"));
}

#[test]
fn test_sign_large_payload() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let large = vec![b'x'; 100000];
    let sig = wh.sign("msg_1", 1700000000, &large).unwrap();
    assert!(sig.starts_with("v1,"));
}

// ═══════════════════════════════════════════════════════════════════
// VERIFY TESTS
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_verify_valid() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let payload = b"{\"event\":\"test\"}";
    let ts = now();
    let sig = wh.sign("msg_1", ts, payload).unwrap();
    let mut headers = http02::HeaderMap::new();
    headers.insert("webhook-id", "msg_1".parse().unwrap());
    headers.insert("webhook-timestamp", ts.to_string().parse().unwrap());
    headers.insert("webhook-signature", sig.parse().unwrap());
    wh.verify(payload, &headers).unwrap();
}

#[test]
fn test_verify_invalid_signature() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let mut headers = http02::HeaderMap::new();
    headers.insert("webhook-id", "msg_1".parse().unwrap());
    headers.insert("webhook-timestamp", now().to_string().parse().unwrap());
    headers.insert("webhook-signature", "v1,invalid".parse().unwrap());
    assert!(wh.verify(b"{}", &headers).is_err());
}

#[test]
fn test_verify_old_timestamp() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let payload = b"{}";
    let ts = now() - 600;
    let sig = wh.sign("msg_1", ts, payload).unwrap();
    let mut headers = http02::HeaderMap::new();
    headers.insert("webhook-id", "msg_1".parse().unwrap());
    headers.insert("webhook-timestamp", ts.to_string().parse().unwrap());
    headers.insert("webhook-signature", sig.parse().unwrap());
    assert!(wh.verify(payload, &headers).is_err());
}

#[test]
fn test_verify_ignoring_timestamp() {
    let wh = super::Webhook::new("whsec_dGVzdA==").unwrap();
    let payload = b"{}";
    let ts = now() - 600;
    let sig = wh.sign("msg_1", ts, payload).unwrap();
    let mut headers = http02::HeaderMap::new();
    headers.insert("webhook-id", "msg_1".parse().unwrap());
    headers.insert("webhook-timestamp", ts.to_string().parse().unwrap());
    headers.insert("webhook-signature", sig.parse().unwrap());
    wh.verify_ignoring_timestamp(payload, &headers).unwrap();
}

fn now() -> i64 {
    std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as i64
}
