//! QR Builder Tests

use payway::{Currency, GenerateQrRequest, PayWayClient, PayWayConfig, PaymentOption, QrItem};

const TEST_API_KEY: &str = "f095f4491513e176cb3b526501d89aa98096d1ba";
const TEST_MERCHANT_ID: &str = "ec474549";

fn create_test_client() -> PayWayClient {
    PayWayClient::with_config(PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY))
}

#[tokio::test]
async fn test_qr_builder_minimal() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-build-min")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.tran_id, "qr-build-min");
    assert_eq!(request.amount, "1.00");
    assert_eq!(request.currency, "USD");
    assert_eq!(request.payment_option, "abapay_khqr");
    assert!(!request.req_time.is_empty());
    assert!(!request.hash.is_empty());
}

#[tokio::test]
async fn test_qr_builder_full() {
    let client = create_test_client();

    let items = vec![QrItem {
        name: "QR Product".to_string(),
        quantity: 2,
        price: 10.00,
        ..Default::default()
    }];

    let custom_fields = serde_json::json!({
        "table": 5,
        "guests": 2
    });

    let payout = vec![serde_json::json!({
        "account": "000133879",
        "amount": 5.00
    })];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-build-full")
        .amount(25.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .purchase_type("purchase")
        .items(items)
        .callback_url("https://example.com/callback")
        .return_deeplink("myapp://return")
        .custom_fields(custom_fields)
        .return_params("order_id=456")
        .payout(payout)
        .lifetime(10)
        .qr_template("template3_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.first_name, Some("John".to_string()));
    assert_eq!(request.last_name, Some("Doe".to_string()));
    assert_eq!(request.email, Some("john@example.com".to_string()));
    assert_eq!(request.phone, Some("012345678".to_string()));
    assert_eq!(request.purchase_type, Some("purchase".to_string()));
    assert!(request.items.is_some());
    assert!(request.callback_url.is_some());
    assert!(request.return_deeplink.is_some());
    assert!(request.custom_fields.is_some());
    assert!(request.payout.is_some());
    assert_eq!(request.lifetime, Some(10));
    assert_eq!(
        request.qr_image_template,
        Some("template3_color".to_string())
    );
}

#[tokio::test]
async fn test_qr_builder_chaining() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-chain")
        .amount(15.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .first_name("Jane")
        .last_name("Smith")
        .email("jane@test.com")
        .phone("0999999999")
        .lifetime(15)
        .qr_template("template2_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.tran_id, "qr-chain");
    assert_eq!(request.amount, "15.00");
    assert_eq!(request.first_name, Some("Jane".to_string()));
    assert_eq!(request.last_name, Some("Smith".to_string()));
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

    assert!(json.contains("\"tran_id\":\"qr-serial\""));
    assert!(json.contains("\"amount\":\"1.00\"")); // String, not number!
    assert!(json.contains("\"currency\":\"USD\""));
    assert!(json.contains("\"hash\":"));
}

#[tokio::test]
async fn test_qr_amount_serialization_string() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-amt-str")
        .amount(10.50)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Amount must be serialized as string "10.50"
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("\"amount\":\"10.50\""));
    assert!(!json.contains("\"amount\":10.50")); // Not a number!
}

#[tokio::test]
async fn test_qr_currency_usd() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-usd")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(request.currency, "USD");
}

#[tokio::test]
async fn test_qr_currency_khr() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-khr")
        .amount(5000.00)
        .currency(Currency::Khr)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // KHR amount should be formatted as integer string
    assert_eq!(request.amount, "5000");
    assert_eq!(request.currency, "KHR");
}

#[tokio::test]
async fn test_qr_items_base64() {
    let client = create_test_client();

    let items = vec![
        QrItem {
            name: "Product A".to_string(),
            quantity: 1,
            price: 10.00,
            ..Default::default()
        },
        QrItem {
            name: "Product B".to_string(),
            quantity: 2,
            price: 5.00,
            ..Default::default()
        },
    ];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-items")
        .amount(20.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .items(items)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Items should be base64 encoded
    let items_b64 = request.items.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&items_b64)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    assert!(decoded_str.contains("Product A"));
    assert!(decoded_str.contains("Product B"));
}

#[tokio::test]
async fn test_qr_callback_url_base64() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-callback")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .callback_url("https://example.com/webhook")
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Callback URL should be base64 encoded
    let callback = request.callback_url.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&callback)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    assert_eq!(decoded_str, "https://example.com/webhook");
}

#[tokio::test]
async fn test_qr_return_deeplink_base64() {
    let client = create_test_client();

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-deeplink")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .return_deeplink("myapp://payment/success")
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Return deeplink should be base64 encoded
    let deeplink = request.return_deeplink.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&deeplink)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    assert!(decoded_str.contains("myapp"));
}

#[tokio::test]
async fn test_qr_custom_fields_base64() {
    let client = create_test_client();

    let custom_fields = serde_json::json!({
        "order_id": "ORD-12345",
        "priority": "high"
    });

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-custom")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .custom_fields(custom_fields)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Custom fields should be base64 encoded
    let custom = request.custom_fields.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&custom)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    assert!(decoded_str.contains("order_id"));
    assert!(decoded_str.contains("ORD-12345"));
}

#[tokio::test]
async fn test_qr_payout_base64() {
    let client = create_test_client();

    let payout = vec![serde_json::json!({
        "account": "000133879",
        "amount": 10.00
    })];

    let request = GenerateQrRequest::builder()
        .transaction_id("qr-payout")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .payout(payout)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)
        .unwrap();

    // Payout should be base64 encoded
    let payout_data = request.payout.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&payout_data)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    assert!(decoded_str.contains("000133879"));
}

#[tokio::test]
async fn test_qr_item_default() {
    let item = QrItem::default();

    assert_eq!(item.name, "");
    assert_eq!(item.quantity, 0);
    assert_eq!(item.price, 0.0);
}

#[tokio::test]
async fn test_qr_item_with_values() {
    let item = QrItem {
        name: "Test Item".to_string(),
        quantity: 3,
        price: 15.50,
        ..Default::default()
    };

    assert_eq!(item.name, "Test Item");
    assert_eq!(item.quantity, 3);
    assert_eq!(item.price, 15.50);
}

#[tokio::test]
async fn test_qr_all_payment_options() {
    let client = create_test_client();

    let options = vec![
        PaymentOption::AbapayKhqr,
        PaymentOption::Wechat,
        PaymentOption::Alipay,
    ];

    for option in options {
        let request = GenerateQrRequest::builder()
            .transaction_id(format!("qr-opt-{}", option.as_str()))
            .amount(1.00)
            .currency(Currency::Usd)
            .payment_option(option)
            .lifetime(5)
            .qr_template("template1_color")
            .build_with_client(&client)
            .unwrap();

        assert!(!request.hash.is_empty());
    }
}
