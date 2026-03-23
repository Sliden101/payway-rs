//! # PayWay Rust SDK
//!
//! Unofficial Rust SDK for ABA PayWay Payment Gateway (Cambodia).
//!
//! ## Features
//!
//! - **Ecommerce Checkout**: Accept payments via cards, ABA Pay, KHQR, WeChat, Alipay
//! - **QR Payments**: Generate dynamic QR codes for ABA KHQR payments
//! - **Credentials on File**: Store payment methods for recurring payments
//! - **Payment Links**: Create and manage payment links
//! - **Payouts**: Split and distribute payments to beneficiaries
//! - **Pre-authorization**: Hold funds and capture later
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use payway::{PayWayClient, Currency};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = PayWayClient::new(
//!         "your_merchant_id",
//!         "your_api_key",
//!     );
//!
//!     // Create a checkout transaction
//!     let params = payway::PurchaseParams::builder()
//!         .transaction_id("order-123")
//!         .amount(10.00)
//!         .currency(Currency::Usd)
//!         .first_name("John")
//!         .last_name("Doe")
//!         .return_url("https://example.com/callback")
//!         .build_with_client(&client)?;
//!
//!     println!("Purchase params created: {:?}", params);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Environment
//!
//! You need to register for a sandbox account at:
//! <https://sandbox.payway.com.kh/register-sandbox/>
//!
//! For production credentials, contact: paywaysales@ababank.com

pub mod api;
pub mod client;
pub mod config;
pub mod constants;
pub mod error;
pub mod types;
pub mod utils;

pub use client::PayWayClient;
pub use config::{Environment, PayWayConfig};
pub use error::{PayWayError, Result};
pub use types::*;
