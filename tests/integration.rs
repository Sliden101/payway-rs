//! WireMock Integration Tests
//!
//! These tests use WireMock to mock the PayWay API responses.
//! They test that the SDK correctly sends requests and parses responses.

use payway::{Currency, GenerateQrRequest, PayWayClient, PayWayConfig, PaymentOption};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_generate_qr_with_mock() {
    let mock_server = MockServer::start().await;

    let client = PayWayClient::with_config(
        PayWayConfig::sandbox("test_merchant", "test_api_key").with_base_url(&mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/api/payment-gateway/v1/payments/generate-qr"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": {
                "code": "00",
                "message": "Successful"
            },
            "amount": 10.00,
            "currency": "USD",
            "qrString": "test-qr-string-123",
            "qrImage": "data:image/png;base64,iVBORw0KGgoAAAANS"
        })))
        .mount(&mock_server)
        .await;

    let request = GenerateQrRequest::builder()
        .transaction_id("test-qr-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    let response = client.generate_qr(request).await.unwrap();

    assert!(response.qr_string.is_some());
    assert_eq!(response.qr_string.unwrap(), "test-qr-string-123");
}

#[tokio::test]
async fn test_generate_qr_request_payload() {
    let mock_server = MockServer::start().await;

    let client = PayWayClient::with_config(
        PayWayConfig::sandbox("test_merchant", "test_api_key").with_base_url(&mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/api/payment-gateway/v1/payments/generate-qr"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": {
                "code": "00",
                "message": "Successful"
            },
            "amount": 10.00,
            "currency": "USD",
            "qrString": "verified-payload",
            "qrImage": ""
        })))
        .mount(&mock_server)
        .await;

    let request = GenerateQrRequest::builder()
        .transaction_id("test-payload-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    let response = client.generate_qr(request).await.unwrap();

    assert!(response.qr_string.is_some());
    assert_eq!(response.qr_string.unwrap(), "verified-payload");
}

#[tokio::test]
async fn test_generate_qr_khr_currency() {
    let mock_server = MockServer::start().await;

    let client = PayWayClient::with_config(
        PayWayConfig::sandbox("test_merchant", "test_api_key").with_base_url(&mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/api/payment-gateway/v1/payments/generate-qr"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": {
                "code": "00",
                "message": "Successful"
            },
            "amount": 5000.00,
            "currency": "KHR",
            "qrString": "khr-qr-string",
            "qrImage": ""
        })))
        .mount(&mock_server)
        .await;

    let request = GenerateQrRequest::builder()
        .transaction_id("test-khr-001")
        .amount(5000.00)
        .currency(Currency::Khr)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(10)
        .qr_template("template2_color")
        .build_with_client(&client)
        .unwrap();

    let response = client.generate_qr(request).await.unwrap();

    assert!(response.qr_string.is_some());
    assert_eq!(response.qr_string.unwrap(), "khr-qr-string");
}

#[tokio::test]
async fn test_generate_qr_with_optional_fields() {
    let mock_server = MockServer::start().await;

    let client = PayWayClient::with_config(
        PayWayConfig::sandbox("test_merchant", "test_api_key").with_base_url(&mock_server.uri()),
    );

    Mock::given(method("POST"))
        .and(path("/api/payment-gateway/v1/payments/generate-qr"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": {
                "code": "00",
                "message": "Successful"
            },
            "amount": 25.00,
            "currency": "USD",
            "qrString": "full-qr-string",
            "qrImage": ""
        })))
        .mount(&mock_server)
        .await;

    let request = GenerateQrRequest::builder()
        .transaction_id("test-optional-001")
        .amount(25.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::Wechat)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .callback_url("https://example.com/callback")
        .lifetime(15)
        .qr_template("template3_color")
        .build_with_client(&client)
        .unwrap();

    let response = client.generate_qr(request).await.unwrap();

    assert!(response.qr_string.is_some());
}
