# PayWay Rust SDK

Unofficial Rust SDK for ABA PayWay Payment Gateway (Cambodia).

[![Crates.io](https://img.shields.io/crates/v/payway)](https://crates.io/crates/payway)
[![Docs](https://docs.rs/payway/badge.svg)](https://docs.rs/payway)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.html)
[![GitHub](https://img.shields.io/github/stars/Sliden101/payway-rs)](https://github.com/Sliden101/payway-rs)

## Features

- QR Code Generation
- Ecommerce Checkout
- Credentials on File (COF)
- Payouts
- Pre-authorization
- Sandbox + Production environments

## Installation

```toml
[dependencies]
payway = "0.1.0"
```

## Quick Start

```rust
use payway::{PayWayClient, Currency};

let client = PayWayClient::new("merchant_id", "api_key");

let params = payway::PurchaseParams::builder()
    .transaction_id("order-123")
    .amount(10.00)
    .currency(Currency::Usd)
    .return_url("https://example.com/callback")
    .build_with_client(&client)?;
```

## Configuration

### Sandbox (Default)

```rust
let client = PayWayClient::new("merchant_id", "api_key");
```

### Production

```rust
let config = PayWayConfig::production("merchant_id", "api_key");
let client = PayWayClient::with_config(config);
```

### Custom Base URL (testing with WireMock)

```rust
let config = PayWayConfig::sandbox("merchant_id", "api_key")
    .with_base_url("http://localhost:8080");
let client = PayWayClient::with_config(config);
```

## API Overview

### Checkout

```rust
let params = PurchaseParams::builder()
    .transaction_id("order-123")
    .amount(10.00)
    .currency(Currency::Usd)
    .first_name("John")
    .last_name("Doe")
    .email("john@example.com")
    .return_url("https://example.com/return")
    .build_with_client(&client)?;
```

### QR Code

```rust
let request = GenerateQrRequest::builder()
    .transaction_id("qr-123")
    .amount(10.00)
    .currency(Currency::Usd)
    .payment_option(PaymentOption::AbapayKhqr)
    .qr_template("template1_color")
    .build_with_client(&client)?;

let response = client.generate_qr(request).await?;
```

## Examples

See `examples/` directory:

- `generate_qr.rs` - Generate QR codes
- `checkout.rs` - Ecommerce checkout

## Testing

```bash
cargo test
```

## License

GNU General Public License v3.0 - See [LICENSE](LICENSE)

## Contributing

Contributions welcome! Please open an issue or PR on [GitHub](https://github.com/Sliden101/payway-rs)
