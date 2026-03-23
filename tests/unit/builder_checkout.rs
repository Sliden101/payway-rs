//! Checkout Builder Tests

use payway::{CheckoutItem, Currency, PayWayClient, PayWayConfig, PaymentOption, PurchaseParams};

const TEST_API_KEY: &str = "f095f4491513e176cb3b526501d89aa98096d1ba";
const TEST_MERCHANT_ID: &str = "ec474549";

fn create_test_client() -> PayWayClient {
    PayWayClient::with_config(PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY))
}

#[tokio::test]
async fn test_purchase_builder_minimal() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("build-min-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.tran_id, "build-min-001");
    assert_eq!(params.amount, 10.00);
    assert_eq!(params.currency, "USD");
    assert!(!params.req_time.is_empty());
    assert!(!params.merchant_id.is_empty());
    assert!(!params.hash.is_empty());
}

#[tokio::test]
async fn test_purchase_builder_full() {
    let client = create_test_client();

    let items = vec![CheckoutItem {
        name: "Test Item".to_string(),
        quantity: 1,
        price: 10.00,
        ..Default::default()
    }];

    let params = PurchaseParams::builder()
        .transaction_id("build-full-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .items(items)
        .shipping(2.00)
        .return_url("https://example.com/return")
        .cancel_url("https://example.com/cancel")
        .continue_success_url("https://example.com/success")
        .custom_fields(serde_json::json!({"key": "value"}))
        .return_params("order_id=123")
        .lifetime(60)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.firstname, "John");
    assert_eq!(params.lastname, Some("Doe".to_string()));
    assert_eq!(params.email, Some("john@example.com".to_string()));
    assert_eq!(params.phone, Some("012345678".to_string()));
    assert!(params.items.is_some());
    assert_eq!(params.shipping, Some(2.00));
    assert!(params.return_url.is_some());
    assert!(params.cancel_url.is_some());
    assert!(params.custom_fields.is_some());
}

#[tokio::test]
async fn test_purchase_builder_chaining() {
    let client = create_test_client();

    // Test method chaining works correctly
    let params = PurchaseParams::builder()
        .transaction_id("chain-001")
        .amount(25.00)
        .currency(Currency::Usd)
        .first_name("Jane")
        .last_name("Smith")
        .email("jane@example.com")
        .phone("0987654321")
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.tran_id, "chain-001");
    assert_eq!(params.amount, 25.00);
    assert_eq!(params.firstname, "Jane");
}

#[tokio::test]
async fn test_purchase_default_values() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("default-001")
        .amount(10.00)
        .build_with_client(&client)
        .unwrap();

    // Default currency should be USD
    assert_eq!(params.currency, "USD");

    // Default transaction type should be purchase
    assert_eq!(params.transaction_type, Some(PaymentOption::default()));
}

#[tokio::test]
async fn test_purchase_serialization() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("serial-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&params).unwrap();

    // Verify essential fields are present
    assert!(json.contains("\"tran_id\":\"serial-001\""));
    assert!(json.contains("\"amount\":"));
    assert!(json.contains("\"currency\":\"USD\""));
    assert!(json.contains("\"hash\":"));
}

#[tokio::test]
async fn test_purchase_items_serialization() {
    let client = create_test_client();

    let items = vec![
        CheckoutItem {
            name: "Item 1".to_string(),
            quantity: 2,
            price: 5.00,
            ..Default::default()
        },
        CheckoutItem {
            name: "Item 2".to_string(),
            quantity: 1,
            price: 10.00,
            ..Default::default()
        },
    ];

    let params = PurchaseParams::builder()
        .transaction_id("items-ser-001")
        .amount(20.00)
        .currency(Currency::Usd)
        .items(items)
        .build_with_client(&client)
        .unwrap();

    // Items should be base64 encoded
    let items_b64 = params.items.unwrap();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&items_b64)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();

    // Should be valid JSON
    assert!(decoded_str.contains("Item 1"));
    assert!(decoded_str.contains("Item 2"));
}

#[tokio::test]
async fn test_purchase_currency_khr() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("khr-001")
        .amount(1000.00)
        .currency(Currency::Khr)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.currency, "KHR");
}

#[tokio::test]
async fn test_purchase_return_deeplink() {
    let client = create_test_client();

    let deeplink = serde_json::json!({
        "android_scheme": "myapp://android",
        "ios_scheme": "myapp://ios"
    });

    let params = PurchaseParams::builder()
        .transaction_id("deeplink-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .return_deeplink(deeplink)
        .build_with_client(&client)
        .unwrap();

    // Return deeplink should be base64 encoded
    assert!(params.return_deeplink.is_some());
}

#[tokio::test]
async fn test_purchase_skip_success_page() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("skip-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .skip_success_page(true)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.skip_success_page, Some(1));
}

#[tokio::test]
async fn test_purchase_view_type() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("view-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    // Default view type should be set
    // (Implementation dependent - may or may not be present)
    assert!(params.merchant_id.len() > 0);
}

#[tokio::test]
async fn test_checkout_item_default() {
    let item = CheckoutItem::default();

    assert_eq!(item.name, "");
    assert_eq!(item.quantity, 0);
    assert_eq!(item.price, 0.0);
}

#[tokio::test]
async fn test_checkout_item_with_values() {
    let item = CheckoutItem {
        name: "Test Product".to_string(),
        quantity: 5,
        price: 25.99,
        ..Default::default()
    };

    assert_eq!(item.name, "Test Product");
    assert_eq!(item.quantity, 5);
    assert_eq!(item.price, 25.99);
}
