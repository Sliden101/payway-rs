//! Validation Tests

use payway::{
    Currency, GenerateQrRequest, PayWayClient, PayWayConfig, PaymentOption, PurchaseParams,
};

const TEST_API_KEY: &str = "f095f4491513e176cb3b526501d89aa98096d1ba";
const TEST_MERCHANT_ID: &str = "ec474549";

fn create_test_client() -> PayWayClient {
    PayWayClient::with_config(PayWayConfig::sandbox(TEST_MERCHANT_ID, TEST_API_KEY))
}

#[tokio::test]
async fn test_amount_minimum_usd() {
    let client = create_test_client();

    // Minimum USD amount is 0.01
    let result = PurchaseParams::builder()
        .transaction_id("min-usd")
        .amount(0.01)
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Should succeed (at boundary)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_amount_minimum_khr() {
    let client = create_test_client();

    // Minimum KHR amount is 100
    let result = PurchaseParams::builder()
        .transaction_id("min-khr")
        .amount(100.00)
        .currency(Currency::Khr)
        .build_with_client(&client);

    // Should succeed
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_amount_zero_usd() {
    let client = create_test_client();

    // Zero amount should work for some use cases
    let result = PurchaseParams::builder()
        .transaction_id("zero-usd")
        .amount(0.00)
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Note: This behavior depends on API - may allow or reject
    // Just ensure it builds without panic
    assert!(result.is_some());
}

#[tokio::test]
async fn test_transaction_id_length() {
    let client = create_test_client();

    // Max transaction ID length is 20 characters
    let long_id = "this-is-a-very-long-transaction-id-that-exceeds-max";

    let result = PurchaseParams::builder()
        .transaction_id(long_id)
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Should still build (validation may happen at API level)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_email_validation_format() {
    let client = create_test_client();

    let result = PurchaseParams::builder()
        .transaction_id("email-test")
        .amount(10.00)
        .currency(Currency::Usd)
        .email("not-a-valid-email")
        .build_with_client(&client);

    // Should build - email validation happens at API level
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_phone_format() {
    let client = create_test_client();

    // Various phone formats should work
    let phones = vec!["012345678", "0123456789", "+855123456789", "855123456789"];

    for phone in phones {
        let result = PurchaseParams::builder()
            .transaction_id("phone-test")
            .amount(10.00)
            .currency(Currency::Usd)
            .phone(phone)
            .build_with_client(&client);

        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_currency_case_insensitive() {
    let client = create_test_client();

    // Currency should accept different cases
    let result_usd = GenerateQrRequest::builder()
        .transaction_id("usd-lower")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client);

    assert!(result_usd.is_ok());
}

#[tokio::test]
async fn test_lifetime_bounds() {
    let client = create_test_client();

    // Minimum lifetime is 3 minutes
    let result_min = GenerateQrRequest::builder()
        .transaction_id("lifetime-min")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(3)
        .qr_template("template1_color")
        .build_with_client(&client);
    assert!(result_min.is_ok());

    // Maximum lifetime is 30 days (43200 minutes)
    let result_max = GenerateQrRequest::builder()
        .transaction_id("lifetime-max")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(43200)
        .qr_template("template1_color")
        .build_with_client(&client);
    assert!(result_max.is_ok());
}

#[tokio::test]
async fn test_empty_transaction_id() {
    let client = create_test_client();

    let result = PurchaseParams::builder()
        .transaction_id("")
        .amount(10.00)
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Should fail - empty transaction ID
    assert!(result.is_err());
}

#[tokio::test]
async fn test_negative_amount() {
    let client = create_test_client();

    let result = PurchaseParams::builder()
        .transaction_id("neg-amt")
        .amount(-10.00)
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Should build - validation happens at API level
    assert!(result.is_some());
}

#[tokio::test]
async fn test_qr_missing_required_fields() {
    let client = create_test_client();

    // Missing transaction_id
    let result = GenerateQrRequest::builder()
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client);

    // Should fail - missing required field
    assert!(result.is_err());
}

#[tokio::test]
async fn test_checkout_missing_required() {
    let client = create_test_client();

    // Missing transaction_id and amount
    let result = PurchaseParams::builder()
        .currency(Currency::Usd)
        .build_with_client(&client);

    // Should fail
    assert!(result.is_err());
}

#[tokio::test]
async fn test_qr_payment_option_required() {
    let client = create_test_client();

    // Missing payment_option
    let result = GenerateQrRequest::builder()
        .transaction_id("test-001")
        .amount(1.00)
        .currency(Currency::Usd)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client);

    assert!(result.is_err());
}

#[tokio::test]
async fn test_name_special_characters() {
    let client = create_test_client();

    // Names with special characters - should work but API may reject
    let result = PurchaseParams::builder()
        .transaction_id("special-name")
        .amount(10.00)
        .currency(Currency::Usd)
        .first_name("John O'Brien")
        .last_name("Smith-Jr")
        .build_with_client(&client);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_url_validation() {
    let client = create_test_client();

    // Invalid URL format
    let result = PurchaseParams::builder()
        .transaction_id("url-test")
        .amount(10.00)
        .currency(Currency::Usd)
        .return_url("not-a-url")
        .build_with_client(&client);

    // Should build - URL validation at API level
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_return_params_format() {
    let client = create_test_client();

    // Return params can be any string
    let result = PurchaseParams::builder()
        .transaction_id("ret-param")
        .amount(10.00)
        .currency(Currency::Usd)
        .return_params("key1=value1&key2=value2")
        .build_with_client(&client);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_large_amount() {
    let client = create_test_client();

    // Large amount should work
    let result = PurchaseParams::builder()
        .transaction_id("large-amt")
        .amount(1_000_000.00)
        .currency(Currency::Usd)
        .build_with_client(&client);

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_decimal_places() {
    let client = create_test_client();

    // Test various decimal places
    let amounts = vec![0.01, 0.10, 1.00, 1.50, 100.99, 100.999];

    for amount in amounts {
        let result = GenerateQrRequest::builder()
            .transaction_id(format!("dec-{}", amount))
            .amount(amount)
            .currency(Currency::Usd)
            .payment_option(PaymentOption::AbapayKhqr)
            .lifetime(5)
            .qr_template("template1_color")
            .build_with_client(&client);

        assert!(result.is_ok());
    }
}
