//! QR Code Generation Example

use base64::Engine;
use payway::{Currency, PayWayClient, PayWayConfig, PaymentOption};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = PayWayConfig::sandbox("merchant-id", "public-key");

    let client = PayWayClient::with_config(config);

    println!("Generating QR code...");
    println!("Merchant ID: {}", client.merchant_id());

    let qr_request = payway::GenerateQrRequest::builder()
        .transaction_id("qr003")
        .amount(1.00)
        .currency(Currency::Usd)
        .payment_option(PaymentOption::AbapayKhqr)
        .lifetime(5)
        .qr_template("template1_color")
        .build_with_client(&client)?;

    println!("\nQR Request created:");
    println!("  Transaction ID: {}", qr_request.tran_id);
    println!("  Amount: {} {}", qr_request.amount, qr_request.currency);
    println!("  Payment Option: {}", qr_request.payment_option);

    let response = client.generate_qr(qr_request).await;

    match response {
        Ok(resp) => {
            println!("\n--- QR Code Generated Successfully ---");
            println!("Status Code: {}", resp.status.code);
            println!("Status Message: {}", resp.status.message);

            if let Some(qr_string) = &resp.qr_string {
                println!("\nQR String (KHQR):");
                println!("  {}", qr_string);
            }

            println!("\nAmount: {} {}", resp.amount, resp.currency);

            if let Some(deeplink) = &resp.abapay_deeplink {
                println!("\nABA Pay Deep Link:");
                println!("  {}", deeplink);
            }

            // Save and display QR image
            if let Some(qr_image) = &resp.qr_image {
                // Remove "data:image/png;base64," prefix if present
                let base64_data = qr_image
                    .strip_prefix("data:image/png;base64,")
                    .unwrap_or(qr_image);

                // Decode base64
                let image_data = base64::engine::general_purpose::STANDARD
                    .decode(base64_data)
                    .expect("Failed to decode base64 image");

                // Save to file
                let filename = "qr_code.png";
                std::fs::write(filename, &image_data)?;
                println!("\nQR code saved to: {}", filename);
                println!("Opening in image viewer...");

                // Open with default image viewer
                std::process::Command::new("xdg-open")
                    .arg(filename)
                    .spawn()
                    .expect("Failed to open image viewer");
            }

            if let Some(trace_id) = &resp.status.trace_id {
                println!("\nTrace ID: {}", trace_id);
            }
        }
        Err(e) => {
            println!("\nFailed to generate QR code: {}", e);
        }
    }

    Ok(())
}
