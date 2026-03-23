//! PayWay Constants

/// Base URL for sandbox environment
pub const BASE_URL_SANDBOX: &str = "https://checkout-sandbox.payway.com.kh/";

/// Base URL for production environment
pub const BASE_URL_PROD: &str = "https://checkout.payway.com.kh/";

/// API version
pub const API_VERSION: &str = "v1";

/// Default request lifetime in minutes
pub const DEFAULT_LIFETIME_MINUTES: u32 = 30 * 24 * 60; // 30 days

/// Minimum lifetime in minutes
pub const MIN_LIFETIME_MINUTES: u32 = 3;

/// Maximum payout beneficiaries per request
pub const MAX_PAYOUT_BENEFICIARIES: usize = 10;

/// Transaction ID length constraints
pub const TRAN_ID_MIN_LENGTH: usize = 1;
pub const TRAN_ID_MAX_LENGTH: usize = 20;

/// Amount constraints
pub const MIN_AMOUNT_USD: f64 = 0.01;
pub const MIN_AMOUNT_KHR: f64 = 100.0;

/// Supported currencies
pub const CURRENCY_USD: &str = "USD";
pub const CURRENCY_KHR: &str = "KHR";

/// Payment options
pub mod payment_option {
    pub const CARDS: &str = "cards";
    pub const ABAPAY_KHQR: &str = "abapay_khqr";
    pub const ABAPAY_KHQR_DEEPLINK: &str = "abapay_khqr_deeplink";
    pub const ALIPAY: &str = "alipay";
    pub const WECHAT: &str = "wechat";
    pub const GOOGLE_PAY: &str = "google_pay";
}

/// Transaction types
pub mod transaction_type {
    pub const PURCHASE: &str = "purchase";
    pub const PRE_AUTH: &str = "pre-auth";
}

/// Transaction statuses
pub mod transaction_status {
    pub const APPROVED: &str = "APPROVED";
    pub const PRE_AUTH: &str = "PRE-AUTH";
    pub const PENDING: &str = "PENDING";
    pub const DECLINED: &str = "DECLINED";
    pub const REFUNDED: &str = "REFUNDED";
    pub const CANCELLED: &str = "CANCELLED";
}

/// Transaction status codes
pub mod status_code {
    pub const SUCCESS: &str = "0";
    pub const SUCCESS_ALT: &str = "00";
    pub const PENDING: &str = "2";
    pub const DECLINED: &str = "3";
    pub const REFUNDED: &str = "4";
    pub const CANCELLED: &str = "7";
}

/// QR image templates
pub mod qr_template {
    pub const TEMPLATE1_COLOR: &str = "template1_color";
    pub const TEMPLATE2_COLOR: &str = "template2_color";
    pub const TEMPLATE3_COLOR: &str = "template3_color";
    pub const TEMPLATE4_COLOR: &str = "template4_color";
    pub const TEMPLATE5_COLOR: &str = "template5_color";
    pub const TEMPLATE6_COLOR: &str = "template6_color";
    pub const TEMPLATE1_BW: &str = "template1_bw";
    pub const TEMPLATE2_BW: &str = "template2_bw";
    pub const TEMPLATE3_BW: &str = "template3_bw";
    pub const TEMPLATE4_BW: &str = "template4_bw";
    pub const TEMPLATE5_BW: &str = "template5_bw";
    pub const TEMPLATE6_BW: &str = "template6_bw";
}

/// API Endpoints
pub mod endpoint {
    /// Ecommerce Checkout endpoints
    pub const PURCHASE: &str = "api/payment-gateway/v1/payments/purchase";
    pub const CHECK_TRANSACTION: &str = "api/payment-gateway/v1/payments/check-transaction-2";
    pub const TRANSACTION_DETAIL: &str = "api/payment-gateway/v1/payments/transaction-detail";
    pub const TRANSACTION_LIST: &str = "api/payment-gateway/v1/payments/transaction-list";
    pub const CLOSE_TRANSACTION: &str = "api/payment-gateway/v1/payments/close-transaction";
    pub const REFUND: &str = "api/payment-gateway/v1/payments/refund";

    /// QR API endpoints
    pub const GENERATE_QR: &str = "api/payment-gateway/v1/payments/generate-qr";

    /// Credentials on File endpoints
    pub const LINK_ACCOUNT: &str = "api/aof/request-qr";
    pub const LINK_CARD: &str = "api/cof/request-card-token";
    pub const PURCHASE_TOKEN: &str = "api/payment-gateway/v1/payments/purchase";

    /// Payment Link endpoints
    pub const CREATE_PAYMENT_LINK: &str =
        "api/merchant-portal/online-transaction/create-payment-link";

    /// Payout endpoints
    pub const ADD_BENEFICIARY: &str =
        "api/merchant-portal/merchant-access/whitelist-account/add-whitelist-payout";
    pub const UPDATE_BENEFICIARY: &str =
        "api/merchant-portal/merchant-access/whitelist-account/update-whitelist-payout-status";
    pub const PAYOUT: &str = "api/payment-gateway/v2/direct-payment/merchant/payout";

    /// Pre-auth endpoints
    pub const PRE_AUTH_COMPLETE: &str =
        "api/merchant-portal/merchant-access/online-transaction/pre-auth-completion";
}
