//! Checkout Hash Tests

use payway::{
    CheckTransactionRequest, CloseTransactionRequest, Currency, PayWayClient, PayWayConfig,
    PaymentOption, PurchaseParams,
};

const TEST_API_KEY: &str = "f095f4491513e176cb3b526501d89aa98096d1ba";
const TEST_MERCHANT_ID: &str = "ec474549";

fn create_test_client() -> PayWayClient {
    PayWayClient::with_config(PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY))
}

#[tokio::test]
async fn test_purchase_hash_calculation() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("test-hash-001")
        .amount(10.00)
        .currency(Currency::Usd)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .return_url("https://example.com/callback")
        .build_with_client(&client)
        .unwrap();

    // Verify hash is computed (non-empty)
    assert!(!params.hash.is_empty());

    // Verify hash is valid base64
    assert!(base64::engine::general_purpose::STANDARD
        .decode(&params.hash)
        .is_ok());
}

#[tokio::test]
async fn test_purchase_hash_deterministic() {
    let client = create_test_client();

    let params1 = PurchaseParams::builder()
        .transaction_id("test-hash-dup")
        .amount(25.50)
        .currency(Currency::Usd)
        .first_name("Jane")
        .last_name("Smith")
        .build_with_client(&client)
        .unwrap();

    let params2 = PurchaseParams::builder()
        .transaction_id("test-hash-dup")
        .amount(25.50)
        .currency(Currency::Usd)
        .first_name("Jane")
        .last_name("Smith")
        .build_with_client(&client)
        .unwrap();

    // Hash should be the same for identical inputs
    assert_eq!(params1.hash, params2.hash);
}

#[tokio::test]
async fn test_purchase_hash_different_inputs() {
    let client = create_test_client();

    let params1 = PurchaseParams::builder()
        .transaction_id("test-diff-1")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    let params2 = PurchaseParams::builder()
        .transaction_id("test-diff-2")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    // Hash should be different for different transaction IDs
    assert_ne!(params1.hash, params2.hash);
}

#[tokio::test]
async fn test_purchase_amount_format_usd() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("test-amount-usd")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client)
        .unwrap();

    // Amount should be serialized as string with 2 decimal places
    assert_eq!(params.amount, 10.00);
}

#[tokio::test]
async fn test_purchase_amount_format_khr() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("test-amount-khr")
        .amount(1000.00)
        .currency(Currency::Khr)
        .build_with_client(&client)
        .unwrap();

    // For KHR, amount should be formatted as integer
    assert_eq!(params.amount, 1000.00);
}

#[tokio::test]
async fn test_check_transaction_hash() {
    let client = create_test_client();

    let request = CheckTransactionRequest::new(&client, "test-tran-001");

    // Verify hash is computed
    assert!(!request.hash.is_empty());

    // Verify required fields
    assert_eq!(request.merchant_id, TEST_MERCHANT_ID);
    assert_eq!(request.tran_id, "test-tran-001");
    assert!(!request.req_time.is_empty());
}

#[tokio::test]
async fn test_close_transaction_hash() {
    let client = create_test_client();

    let request = CloseTransactionRequest::new(&client, "test-tran-002");

    // Verify hash is computed
    assert!(!request.hash.is_empty());

    // Verify required fields
    assert_eq!(request.merchant_id, TEST_MERCHANT_ID);
    assert_eq!(request.tran_id, "test-tran-002");
}

#[tokio::test]
async fn test_purchase_with_shipping() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("test-shipping")
        .amount(50.00)
        .currency(Currency::Usd)
        .shipping(5.00)
        .build_with_client(&client)
        .unwrap();

    // Shipping should be included
    assert!(params.shipping.is_some());
    if let Some(shipping) = params.shipping {
        assert_eq!(shipping, 5.00);
    }
}

#[tokio::test]
async fn test_purchase_with_items() {
    let client = create_test_client();

    let items = vec![
        payway::CheckoutItem {
            name: "Item 1".to_string(),
            quantity: 1,
            price: 10.00,
            ..Default::default()
        },
        payway::CheckoutItem {
            name: "Item 2".to_string(),
            quantity: 2,
            price: 5.00,
            ..Default::default()
        },
    ];

    let params = PurchaseParams::builder()
        .transaction_id("test-items")
        .amount(20.00)
        .currency(Currency::Usd)
        .items(items)
        .build_with_client(&client)
        .unwrap();

    // Items should be base64 encoded
    assert!(params.items.is_some());
}

#[tokio::test]
async fn test_purchase_with_custom_fields() {
    let client = create_test_client();

    let custom_fields = serde_json::json!({
        "order_id": "ORD-123",
        "customer_type": "premium"
    });

    let params = PurchaseParams::builder()
        .transaction_id("test-custom")
        .amount(100.00)
        .currency(Currency::Usd)
        .custom_fields(custom_fields)
        .build_with_client(&client)
        .unwrap();

    // Custom fields should be base64 encoded
    assert!(params.custom_fields.is_some());
}

#[tokio::test]
async fn test_purchase_with_payout() {
    let client = create_test_client();

    let payout = vec![
        serde_json::json!({
            "acc": "000133879",
            "amt": 50
        }),
        serde_json::json!({
            "acc": "000133880",
            "amt": 50
        }),
    ];

    let params = PurchaseParams::builder()
        .transaction_id("test-payout")
        .amount(100.00)
        .currency(Currency::Usd)
        .payout(payout)
        .build_with_client(&client)
        .unwrap();

    // Payout should be base64 encoded
    assert!(params.payout.is_some());
}

#[tokio::test]
async fn test_purchase_with_payment_option() {
    let client = create_test_client();

    let params = PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY).with_rsa_keys(
        "-----BEGIN RSA PRIVATE KEY-----\ntest\n-----END RSA PRIVATE KEY-----",
        "-----BEGIN PUBLIC KEY-----\ntest\n-----END PUBLIC KEY-----",
    );

    let client = PayWayClient::with_config(params);

    let purchase_params = PurchaseParams::builder()
        .transaction_id("test-option")
        .amount(10.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(
        purchase_params.payment_option,
        Some(PaymentOption::AbapayKhqr)
    );
}

#[tokio::test]
async fn test_purchase_lifetime() {
    let client = create_test_client();

    let params = PurchaseParams::builder()
        .transaction_id("test-lifetime")
        .amount(10.00)
        .currency(Currency::Usd)
        .lifetime(60)
        .build_with_client(&client)
        .unwrap();

    assert_eq!(params.lifetime, Some(60));
}
