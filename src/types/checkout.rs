//! Ecommerce Checkout Types

use serde::{Deserialize, Serialize};

use super::{ApiStatus, Currency, PaymentOption, PayoutItem, QrItem, TransactionType};
use crate::client::PayWayClient;
use crate::error::Result;
use crate::utils::hash::{encode_base64, encode_json_base64, generate_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApiStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abapay_deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_qr_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTransactionRequest {
    pub req_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    pub hash: String,
}

impl CheckTransactionRequest {
    pub fn new(client: &PayWayClient, tran_id: impl Into<String>) -> Self {
        let req_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let tran_id = tran_id.into();

        let hash = generate_hash(client.api_key(), &[&req_time, &merchant_id, &tran_id]);

        Self {
            req_time,
            merchant_id,
            tran_id,
            hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckTransactionResponse {
    pub data: Option<TransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub payment_status_code: i32,
    pub total_amount: f64,
    pub original_amount: f64,
    pub refund_amount: f64,
    pub discount_amount: f64,
    pub payment_amount: f64,
    pub payment_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apv: Option<String>,
    pub payment_status: String,
    pub transaction_date: String,
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetailRequest {
    pub req_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    pub hash: String,
}

impl TransactionDetailRequest {
    pub fn new(client: &PayWayClient, tran_id: impl Into<String>) -> Self {
        let req_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let tran_id = tran_id.into();

        let hash = generate_hash(client.api_key(), &[&req_time, &merchant_id, &tran_id]);

        Self {
            req_time,
            merchant_id,
            tran_id,
            hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetailResponse {
    pub data: Option<TransactionDetailData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetailData {
    pub transaction_id: String,
    pub payment_status_code: i32,
    pub payment_status: String,
    pub original_amount: f64,
    pub original_currency: String,
    pub payment_amount: f64,
    pub payment_currency: String,
    pub total_amount: f64,
    pub refund_amount: f64,
    pub discount_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apv: Option<String>,
    pub transaction_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_operations: Option<Vec<TransactionOperation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOperation {
    pub status: String,
    pub amount: f64,
    pub transaction_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseTransactionRequest {
    pub req_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    pub hash: String,
}

impl CloseTransactionRequest {
    pub fn new(client: &PayWayClient, tran_id: impl Into<String>) -> Self {
        let req_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let tran_id = tran_id.into();

        let hash = generate_hash(client.api_key(), &[&req_time, &merchant_id, &tran_id]);

        Self {
            req_time,
            merchant_id,
            tran_id,
            hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseTransactionResponse {
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    pub request_time: String,
    pub merchant_id: String,
    pub merchant_auth: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    pub grand_total: f64,
    pub total_refunded: f64,
    pub currency: String,
    pub transaction_status: String,
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseParams {
    pub req_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    pub firstname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<f64>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_success_page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_params: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay_token: Option<String>,
    pub hash: String,
}

impl PurchaseParams {
    pub fn builder() -> PurchaseParamsBuilder {
        PurchaseParamsBuilder::new()
    }
}

pub struct PurchaseParamsBuilder {
    req_time: String,
    merchant_id: String,
    tran_id: String,
    firstname: String,
    lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    transaction_type: Option<String>,
    payment_option: Option<String>,
    items: Option<Vec<QrItem>>,
    shipping: Option<f64>,
    amount: f64,
    currency: Option<String>,
    return_url: Option<String>,
    cancel_url: Option<String>,
    skip_success_page: Option<i32>,
    continue_success_url: Option<String>,
    return_deeplink: Option<String>,
    custom_fields: Option<serde_json::Value>,
    return_params: Option<String>,
    payout: Option<Vec<PayoutItem>>,
    lifetime: Option<i32>,
    google_pay_token: Option<String>,
}

impl PurchaseParamsBuilder {
    pub fn new() -> Self {
        Self {
            req_time: PayWayClient::generate_request_time(),
            merchant_id: String::new(),
            tran_id: String::new(),
            firstname: String::new(),
            lastname: None,
            email: None,
            phone: None,
            transaction_type: None,
            payment_option: None,
            items: None,
            shipping: None,
            amount: 0.0,
            currency: None,
            return_url: None,
            cancel_url: None,
            skip_success_page: None,
            continue_success_url: None,
            return_deeplink: None,
            custom_fields: None,
            return_params: None,
            payout: None,
            lifetime: None,
            google_pay_token: None,
        }
    }

    pub fn merchant_id(mut self, merchant_id: impl Into<String>) -> Self {
        self.merchant_id = merchant_id.into();
        self
    }

    pub fn transaction_id(mut self, tran_id: impl Into<String>) -> Self {
        self.tran_id = tran_id.into();
        self
    }

    pub fn first_name(mut self, firstname: impl Into<String>) -> Self {
        self.firstname = firstname.into();
        self
    }

    pub fn last_name(mut self, lastname: impl Into<String>) -> Self {
        self.lastname = Some(lastname.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = amount;
        self
    }

    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency.as_str().to_string());
        self
    }

    pub fn payment_option(mut self, option: PaymentOption) -> Self {
        self.payment_option = Some(option.as_str().to_string());
        self
    }

    pub fn items(mut self, items: Vec<QrItem>) -> Self {
        self.items = Some(items);
        self
    }

    pub fn shipping(mut self, shipping: f64) -> Self {
        self.shipping = Some(shipping);
        self
    }

    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(encode_base64(url.into()));
        self
    }

    pub fn cancel_url(mut self, url: impl Into<String>) -> Self {
        self.cancel_url = Some(encode_base64(url.into()));
        self
    }

    pub fn return_params(mut self, params: impl Into<String>) -> Self {
        self.return_params = Some(params.into());
        self
    }

    pub fn skip_success_page(mut self, skip: bool) -> Self {
        self.skip_success_page = Some(if skip { 1 } else { 0 });
        self
    }

    pub fn continue_success_url(mut self, url: impl Into<String>) -> Self {
        self.continue_success_url = Some(url.into());
        self
    }

    pub fn payout(mut self, items: Vec<PayoutItem>) -> Self {
        self.payout = Some(items);
        self
    }

    pub fn lifetime(mut self, minutes: i32) -> Self {
        self.lifetime = Some(minutes);
        self
    }

    pub fn custom_fields(mut self, fields: serde_json::Value) -> Self {
        self.custom_fields = Some(fields);
        self
    }

    pub fn transaction_type(mut self, ttype: TransactionType) -> Self {
        self.transaction_type = Some(ttype.as_str().to_string());
        self
    }

    pub fn build_with_client(self, client: &PayWayClient) -> Result<PurchaseParams> {
        let api_key = client.api_key();

        let merchant_id = if self.merchant_id.is_empty() {
            client.merchant_id().to_string()
        } else {
            self.merchant_id
        };

        let items_b64 = match &self.items {
            Some(items) => Some(encode_json_base64(items)?),
            None => None,
        };

        let payout_b64 = match &self.payout {
            Some(items) => Some(encode_json_base64(items)?),
            None => None,
        };

        let custom_fields_b64 = match &self.custom_fields {
            Some(fields) => Some(encode_json_base64(fields)?),
            None => None,
        };

        let amount_str = match self.currency.as_deref() {
            Some("KHR") => format!("{}", self.amount.round() as i64),
            _ => format!("{:.2}", self.amount),
        };
        let shipping_str = match self.currency.as_deref() {
            Some("KHR") => self
                .shipping
                .map(|s| format!("{}", s.round() as i64))
                .unwrap_or_default(),
            _ => self
                .shipping
                .map(|s| format!("{:.2}", s))
                .unwrap_or_default(),
        };
        let lifetime_str = self.lifetime.map(|l| l.to_string()).unwrap_or_default();
        let skip_success_str = self
            .skip_success_page
            .clone()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let fields_for_hash = vec![
            &self.req_time,
            &merchant_id,
            &self.tran_id,
            &amount_str,
            items_b64.as_deref().unwrap_or(""),
            &shipping_str,
            &self.firstname,
            self.lastname.as_deref().unwrap_or(""),
            self.email.as_deref().unwrap_or(""),
            self.phone.as_deref().unwrap_or(""),
            self.transaction_type.as_deref().unwrap_or(""),
            self.payment_option.as_deref().unwrap_or(""),
            self.return_url.as_deref().unwrap_or(""),
            self.cancel_url.as_deref().unwrap_or(""),
            self.continue_success_url.as_deref().unwrap_or(""),
            self.return_deeplink.as_deref().unwrap_or(""),
            self.currency.as_deref().unwrap_or(""),
            custom_fields_b64.as_deref().unwrap_or(""),
            self.return_params.as_deref().unwrap_or(""),
            payout_b64.as_deref().unwrap_or(""),
            &lifetime_str,
            "",
            self.google_pay_token.as_deref().unwrap_or(""),
            &skip_success_str,
        ];

        let hash = generate_hash(api_key, &fields_for_hash);

        Ok(PurchaseParams {
            req_time: self.req_time,
            merchant_id,
            tran_id: self.tran_id,
            firstname: self.firstname,
            lastname: self.lastname,
            email: self.email,
            phone: self.phone,
            transaction_type: self.transaction_type,
            payment_option: self.payment_option,
            items: items_b64,
            shipping: self.shipping,
            amount: self.amount,
            currency: self.currency,
            return_url: self.return_url,
            cancel_url: self.cancel_url,
            skip_success_page: self.skip_success_page,
            continue_success_url: self.continue_success_url,
            return_deeplink: self.return_deeplink,
            custom_fields: custom_fields_b64,
            return_params: self.return_params,
            payout: payout_b64,
            lifetime: self.lifetime,
            google_pay_token: self.google_pay_token,
            hash,
        })
    }
}

impl Default for PurchaseParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
