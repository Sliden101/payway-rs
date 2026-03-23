//! QR Hash and Builder Tests

use payway::{Currency, GenerateQrRequest, PayWayClient, PayWayConfig, PaymentOption, QrItem};

const TEST_API_KEY: &str = "f095f4491513e176cb3b526501d89aa98096d1ba";
const TEST_MERCHANT_ID: &str = "ec474549";

fn create_test_client() -> PayWayClient {
    PayWayClient::with_config(PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY))
}

#[tokio::test]
async fn test_qr_hash_calculation() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-test-001")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert!(!request.hash.is_empty());
    use base64::Engine;
    assert!(base64::engine::general_purpose::STANDARD
        .decode(&request.hash)
        .is_ok());
}

#[tokio::test]
async fn test_qr_hash_deterministic() {
    let client = create_test_client();

    let request1 = GenerateQrRequest::builder()
        .transaction_id("qr-det-001")
        .amount(5.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(10)
        .qr_template("template2_color")
        .build_with_client(&client)
        .unwrap();

    let request2 = GenerateQrRequest::builder()
        .transaction_id("qr-det-001")
        .amount(5.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(10)
        .qr_template("template2_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request1.hash, request2.hash);
}

#[tokio::test]
async fn test_qr_amount_string_serialization() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-str-amount")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.amount, "1.00");
}

#[tokio::test]
async fn test_qr_amount_khr() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-khr-001")
        .amount(5000.00)
        .currency(Currency::Khr)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.amount, "5000");
}

#[tokio::test]
async fn test_qr_with_all_optional_fields() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-full-001")
        .amount(25.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .callback_url("https://example.com/callback")
        .lifetime(10)
        .qr_template("template3_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.first_name, Some("John".to_string()));
    assert_eq!(request.last_name, Some("Doe".to_string()));
    assert_eq!(request.email, Some("john@example.com".to_string()));
}

#[tokio::test]
async fn test_qr_minimal_required_fields() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-min-001")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.tran_id, "qr-min-001");
    assert_eq!(request.amount, "1.00");
    assert_eq!(request.currency, "USD");
}

#[tokio::test]
async fn test_qr_with_items() {
    let client = create_test_client();

    let items = vec![QrItem {
        name: "Product 1".to_string(),
        quantity: 2,
        price: 10.00,
    }];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-items-001")
        .amount(20.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .items(items)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert!(request.items.is_some());
}

#[tokio::test]
async fn test_qr_items_base64() {
    let client = create_test_client();

    let items = vec![QrItem {
        name: "Product A".to_string(),
        quantity: 1,
        price: 10.00,
    }];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-items-b64")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .items(items)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    use base64::Engine;
    let items_b64 = request.items.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&items_b64)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    assert!(decoded_str.contains("Product A"));
}

#[tokio::test]
async fn test_qr_serialization() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-serial")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"amount\":\"1.00\""));
}

#[tokio::test]
async fn test_qr_payment_options() {
    let client = create_test_client();

    let request1 = GenerateQrRequest::builder()
        .transaction_id("qr-opt-1")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();
    assert_eq!(request1.payment_option, "abapay_khqr");

    let request2 = GenerateQrRequest::builder()
        .transaction_id("qr-opt-2")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::Wechat)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();
    assert_eq!(request2.payment_option, "wechat");
}
