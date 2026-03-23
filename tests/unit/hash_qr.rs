//! QR Hash Tests

use payway::{Currency, GenerateQrRequest, PayWayClient, PayWayConfig, PaymentOption};

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

    // Verify hash is computed (non-empty)
    assert!(!request.hash.is_empty());

    // Verify hash is valid base64
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

    // Hash should be the same for identical inputs
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

    // Amount should be serialized as string "1.00"
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

    // For KHR, amount should be formatted as integer string
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
        .return_deeplink("myapp://return")
        .lifetime(10)
        .qr_template("template3_color")
        .build_with_client(&client)
        .unwrap();

    // All fields should be present
    assert_eq!(request.first_name, Some("John".to_string()));
    assert_eq!(request.last_name, Some("Doe".to_string()));
    assert_eq!(request.email, Some("john@example.com".to_string()));
    assert_eq!(request.phone, Some("012345678".to_string()));
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

    // Minimal required fields should be present
    assert_eq!(request.tran_id, "qr-min-001");
    assert_eq!(request.amount, "1.00");
    assert_eq!(request.currency, "USD");
    assert_eq!(request.payment_option, "abapay_khqr");
    assert_eq!(request.lifetime, Some(5));
    assert_eq!(
        request.qr_image_template,
        Some("template1_color".to_string())
    );
}

#[tokio::test]
async fn test_qr_with_items() {
    let client = create_test_client();

    let items = vec![payway::QrItem {
        name: "Product 1".to_string(),
        quantity: 2,
        price: 10.00,
        ..Default::default()
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

    // Items should be base64 encoded JSON
    assert!(request.items.is_some());
}

#[tokio::test]
async fn test_qr_with_custom_fields() {
    let client = create_test_client();

    let custom_fields = serde_json::json!({
        "table_number": 5,
        "guest_count": 4
    });

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-custom-001")
        .amount(50.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .custom_fields(custom_fields)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Custom fields should be base64 encoded
    assert!(request.custom_fields.is_some());
}

#[tokio::test]
async fn test_qr_with_payout() {
    let client = create_test_client();

    let payout = vec![serde_json::json!({
        "account": "000133879",
        "amount": 10.00
    })];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-payout-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .payout(payout)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Payout should be base64 encoded
    assert!(request.payout.is_some());
}

#[tokio::test]
async fn test_qr_payment_options() {
    let client = create_test_client();

    // Test abapay_khqr
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

    // Test wechat
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

    // Test alipay
    let request3 = GenerateQrRequest::builder()
        .transaction_id("qr-opt-3")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::Alipay)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();
    assert_eq!(request3.payment_option, "alipay");
}

#[tokio::test]
async fn test_qr_templates() {
    let client = create_test_client();

    // Test different templates
    for template in vec!["template1_color", "template2_color", "template1_bw"] {
        let request = GenerateQrRequest::builder()
            .transaction_id(format!("qr-tpl-{}", template))
            .amount(1.00)
            .currency(Currency::Usd)
            .payment_option(PaymentOption::AbapayKhqr)
            .qr_template(template)
            .lifetime(5)
            .build_with_client(&client)
            .unwrap();

        assert_eq!(request.qr_image_template, Some(template.to_string()));
    }
}

#[tokio::test]
async fn test_qr_different_amounts_same_hash_order() {
    let client = create_test_client();

    let amounts = vec![0.01, 1.00, 10.00, 100.00, 1000.00];

    for (i, amount) in amounts.iter().enumerate() {
        let request = GenerateQrRequest::builder()
            .transaction_id(format!("qr-amt-{}", i))
            .amount(*amount)
            .currency(Currency::Usd)
            .payment_option(PaymentOption::AbapayKhqr)
            .lifetime(5)
            .qr_template("template1_color")
            .build_with_client(&client)
            .unwrap();

        // Each should produce a valid hash
        assert!(!request.hash.is_empty());
        // Hash should be valid base64
        assert!(base64::engine::general_purpose::STANDARD
            .decode(&request.hash)
            .is_ok());
    }
}
