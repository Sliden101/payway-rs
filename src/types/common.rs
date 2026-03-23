//! Common Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStatus {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl ApiStatus {
    pub fn is_success(&self) -> bool {
        self.code == "0" || self.code == "00"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "KHR")]
    Khr,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::Usd => "USD",
            Currency::Khr => "KHR",
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    #[serde(rename = "purchase")]
    Purchase,
    #[serde(rename = "pre-auth")]
    PreAuth,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Purchase => "purchase",
            TransactionType::PreAuth => "pre-auth",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "PRE-AUTH")]
    PreAuth,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "REFUNDED")]
    Refunded,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl PaymentStatus {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "0" | "00" => Some(PaymentStatus::Approved),
            "2" => Some(PaymentStatus::Pending),
            "3" => Some(PaymentStatus::Declined),
            "4" => Some(PaymentStatus::Refunded),
            "7" => Some(PaymentStatus::Cancelled),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentStatus::Approved => "APPROVED",
            PaymentStatus::PreAuth => "PRE-AUTH",
            PaymentStatus::Pending => "PENDING",
            PaymentStatus::Declined => "DECLINED",
            PaymentStatus::Refunded => "REFUNDED",
            PaymentStatus::Cancelled => "CANCELLED",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentOption {
    #[serde(rename = "cards")]
    Cards,
    #[serde(rename = "abapay_khqr")]
    AbapayKhqr,
    #[serde(rename = "abapay_khqr_deeplink")]
    AbapayKhqrDeeplink,
    #[serde(rename = "alipay")]
    Alipay,
    #[serde(rename = "wechat")]
    Wechat,
    #[serde(rename = "google_pay")]
    GooglePay,
}

impl PaymentOption {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentOption::Cards => "cards",
            PaymentOption::AbapayKhqr => "abapay_khqr",
            PaymentOption::AbapayKhqrDeeplink => "abapay_khqr_deeplink",
            PaymentOption::Alipay => "alipay",
            PaymentOption::Wechat => "wechat",
            PaymentOption::GooglePay => "google_pay",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutItem {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrItem {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeeplinkScheme {
    pub ios_scheme: Option<String>,
    pub android_scheme: Option<String>,
}
