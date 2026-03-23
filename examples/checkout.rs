//! Basic Checkout Example

use payway::{Currency, PayWayClient, PayWayConfig, PaymentOption};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = PayWayConfig::sandbox("merchant-id", "public-key");

    let client = PayWayClient::with_config(config);

    println!("Creating a checkout transaction...");
    println!("Merchant ID: {}", client.merchant_id());
    println!("Environment: {}", client.environment());

    let purchase_params = payway::PurchaseParams::builder()
        .transaction_id("order-123456")
        .amount(10.00)
        .currency(Currency::Usd)
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone("012345678")
        .return_url("https://example.com/callback")
        .cancel_url("https://example.com/cancel")
        .payment_option(PaymentOption::Cards)
        .build_with_client(&client)?;

    println!("\nPurchase params created successfully");
    println!("Transaction ID: {}", purchase_params.tran_id);
    println!(
        "Amount: {} {}",
        purchase_params.amount,
        purchase_params
            .currency
            .as_ref()
            .unwrap_or(&"USD".to_string())
    );

    let response = client.create_transaction(purchase_params).await;

    match response {
        Ok(resp) => {
            println!("\nTransaction created successfully!");
            println!("Response: {:?}", resp);
        }
        Err(e) => {
            println!("\nFailed to create transaction: {}", e);
        }
    }

    println!("\nChecking transaction status...");
    let check_result = client.check_transaction("order-123456").await;

    match check_result {
        Ok(result) => {
            println!("Transaction status: {:?}", result);
        }
        Err(e) => {
            println!("Failed to check transaction: {}", e);
        }
    }

    Ok(())
}
