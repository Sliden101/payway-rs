//! Checkout Hash Tests
//!
//! These tests verify hash calculation for checkout API endpoints

use payway::{
    CheckTransactionRequest, CloseTransactionRequest, Currency, PayWayClient, PayWayConfig,
    PayoutItem, PurchaseParams, QrItem,
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

    assert!(!params.hash.is_empty());
    use base64::Engine;
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

    assert_ne!(params1.hash, params2.hash);
}

#[tokio::test]
async fn test_check_transaction_hash() {
    let client = create_test_client();

    let request = CheckTransactionRequest::new(&client, "test-tran-001");

    assert!(!request.hash.is_empty());
    assert_eq!(request.merchant_id, TEST_MERCHANT_ID);
    assert_eq!(request.tran_id, "test-tran-001");
}

#[tokio::test]
async fn test_close_transaction_hash() {
    let client = create_test_client();

    let request = CloseTransactionRequest::new(&client, "test-tran-002");

    assert!(!request.hash.is_empty());
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

    assert!(params.shipping.is_some());
}

#[tokio::test]
async fn test_purchase_with_items() {
    let client = create_test_client();

    let items = vec![
        QrItem {
            name: "Item 1".to_string(),
            quantity: 1,
            price: 10.00,
        },
        QrItem {
            name: "Item 2".to_string(),
            quantity: 2,
            price: 5.00,
        },
    ];

    let params = PurchaseParams::builder()
        .transaction_id("test-items")
        .amount(20.00)
        .currency(Currency::Usd)
        .items(items)
        .build_with_client(&client)
        .unwrap();

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

    assert!(params.custom_fields.is_some());
}

#[tokio::test]
async fn test_purchase_with_payout() {
    let client = create_test_client();

    let payout = vec![
        PayoutItem {
            account: "000133879".to_string(),
            amount: 50.0,
        },
        PayoutItem {
            account: "000133880".to_string(),
            amount: 50.0,
        },
    ];

    let params = PurchaseParams::builder()
        .transaction_id("test-payout")
        .amount(100.00)
        .currency(Currency::Usd)
        .payout(payout)
        .build_with_client(&client)
        .unwrap();

    assert!(params.payout.is_some());
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
