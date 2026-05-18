#[cfg(test)]
mod tests {
    use hooksniff::webhooks::Webhook;

    /// Returns a recent timestamp (seconds since epoch)
    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    #[test]
    fn test_webhook_verify_valid() {
        let secret = "whsec_dGVzdA==";
        let payload = r#"{"event":"test"}"#;

        let wh = Webhook::new(secret).unwrap();

        let timestamp = now();
        let msg_id = "msg_test123";
        let signature = wh.sign(msg_id, timestamp, payload.as_bytes()).unwrap();

        let mut headers = http02::HeaderMap::new();
        headers.insert("webhook-id", msg_id.parse().unwrap());
        headers.insert("webhook-timestamp", timestamp.to_string().parse().unwrap());
        headers.insert("webhook-signature", signature.parse().unwrap());

        wh.verify(payload.as_bytes(), &headers).unwrap();
    }

    #[test]
    fn test_webhook_reject_invalid_signature() {
        let secret = "whsec_dGVzdA==";
        let payload = r#"{"event":"test"}"#;

        let wh = Webhook::new(secret).unwrap();

        let mut headers = http02::HeaderMap::new();
        headers.insert("webhook-id", "msg_test123".parse().unwrap());
        headers.insert("webhook-timestamp", now().to_string().parse().unwrap());
        headers.insert("webhook-signature", "v1,invalid".parse().unwrap());

        let result = wh.verify(payload.as_bytes(), &headers);
        assert!(result.is_err());
    }

    #[test]
    fn test_webhook_hooksniff_branded_headers() {
        let secret = "whsec_dGVzdA==";
        let payload = r#"{"event":"test"}"#;

        let wh = Webhook::new(secret).unwrap();

        let timestamp = now();
        let msg_id = "msg_test123";
        let signature = wh.sign(msg_id, timestamp, payload.as_bytes()).unwrap();

        let mut headers = http02::HeaderMap::new();
        headers.insert("hooksniff-id", msg_id.parse().unwrap());
        headers.insert("hooksniff-timestamp", timestamp.to_string().parse().unwrap());
        headers.insert("hooksniff-signature", signature.parse().unwrap());

        wh.verify(payload.as_bytes(), &headers).unwrap();
    }

    #[test]
    fn test_error_types() {
        let err = hooksniff::error::Error::Generic("test error".to_string());
        assert!(format!("{err}").contains("test error"));

        let http_err = hooksniff::error::Error::Http(hooksniff::error::HttpErrorContent {
            status: http02::StatusCode::BAD_REQUEST,
            headers: None,
            payload: None,
        });
        assert!(format!("{http_err}").contains("400"));
    }
}
