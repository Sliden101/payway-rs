//! QR Payment Types

use serde::{Deserialize, Serialize};

use super::{ApiStatus, Currency, PaymentOption, QrItem};
use crate::client::PayWayClient;
use crate::utils::hash::{encode_base64, encode_json_base64, generate_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateQrRequest {
    pub req_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub amount: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<String>,
    pub payment_option: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
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
    pub qr_image_template: Option<String>,
    pub hash: String,
}

impl GenerateQrRequest {
    pub fn builder() -> GenerateQrRequestBuilder {
        GenerateQrRequestBuilder::new()
    }
}

pub struct GenerateQrRequestBuilder {
    req_time: String,
    merchant_id: String,
    tran_id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    amount: f64,
    currency: String,
    purchase_type: Option<String>,
    payment_option: PaymentOption,
    items: Option<Vec<QrItem>>,
    callback_url: Option<String>,
    return_deeplink: Option<String>,
    custom_fields: Option<serde_json::Value>,
    return_params: Option<String>,
    payout: Option<serde_json::Value>,
    lifetime: Option<i32>,
    qr_image_template: Option<String>,
}

impl GenerateQrRequestBuilder {
    pub fn new() -> Self {
        Self {
            req_time: PayWayClient::generate_request_time(),
            merchant_id: String::new(),
            tran_id: String::new(),
            first_name: None,
            last_name: None,
            email: None,
            phone: None,
            amount: 0.0,
            currency: "USD".to_string(),
            purchase_type: None,
            payment_option: PaymentOption::AbapayKhqr,
            items: None,
            callback_url: None,
            return_deeplink: None,
            custom_fields: None,
            return_params: None,
            payout: None,
            lifetime: None,
            qr_image_template: None,
        }
    }

    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = id.into();
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.tran_id = id.into();
        self
    }

    pub fn first_name(mut self, name: impl Into<String>) -> Self {
        self.first_name = Some(name.into());
        self
    }

    pub fn last_name(mut self, name: impl Into<String>) -> Self {
        self.last_name = Some(name.into());
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
        self.currency = currency.as_str().to_string();
        self
    }

    pub fn payment_option(mut self, option: PaymentOption) -> Self {
        self.payment_option = option;
        self
    }

    pub fn items(mut self, items: Vec<QrItem>) -> Self {
        self.items = Some(items);
        self
    }

    pub fn callback_url(mut self, url: impl Into<String>) -> Self {
        self.callback_url = Some(encode_base64(url.into()));
        self
    }

    pub fn return_params(mut self, params: impl Into<String>) -> Self {
        self.return_params = Some(params.into());
        self
    }

    pub fn payout(mut self, payout: serde_json::Value) -> Self {
        self.payout = Some(payout);
        self
    }

    pub fn lifetime(mut self, minutes: i32) -> Self {
        self.lifetime = Some(minutes);
        self
    }

    pub fn qr_template(mut self, template: impl Into<String>) -> Self {
        self.qr_image_template = Some(template.into());
        self
    }

    pub fn build_with_client(
        self,
        client: &PayWayClient,
    ) -> Result<GenerateQrRequest, crate::error::PayWayError> {
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
            Some(p) => Some(encode_json_base64(p)?),
            None => None,
        };

        let custom_fields_b64 = match &self.custom_fields {
            Some(f) => Some(encode_json_base64(f)?),
            None => None,
        };

        let amount_str = match self.currency.as_str() {
            "KHR" => format!("{}", self.amount.round() as i64),
            _ => format!("{:.2}", self.amount),
        };
        let lifetime_str = self.lifetime.map(|l| l.to_string()).unwrap_or_default();
        let payment_option_str = self.payment_option.as_str().to_string();

        let fields_for_hash = vec![
            &self.req_time,
            &merchant_id,
            &self.tran_id,
            &amount_str,
            items_b64.as_deref().unwrap_or(""),
            self.first_name.as_deref().unwrap_or(""),
            self.last_name.as_deref().unwrap_or(""),
            self.email.as_deref().unwrap_or(""),
            self.phone.as_deref().unwrap_or(""),
            self.purchase_type.as_deref().unwrap_or(""),
            &payment_option_str,
            self.callback_url.as_deref().unwrap_or(""),
            self.return_deeplink.as_deref().unwrap_or(""),
            &self.currency,
            custom_fields_b64.as_deref().unwrap_or(""),
            self.return_params.as_deref().unwrap_or(""),
            payout_b64.as_deref().unwrap_or(""),
            &lifetime_str,
            self.qr_image_template.as_deref().unwrap_or(""),
        ];

        let hash = generate_hash(api_key, &fields_for_hash);

        Ok(GenerateQrRequest {
            req_time: self.req_time,
            merchant_id,
            tran_id: self.tran_id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            phone: self.phone,
            amount: amount_str,
            currency: self.currency,
            purchase_type: self.purchase_type,
            payment_option: self.payment_option.as_str().to_string(),
            items: items_b64,
            callback_url: self.callback_url,
            return_deeplink: self.return_deeplink,
            custom_fields: custom_fields_b64,
            return_params: self.return_params,
            payout: payout_b64,
            lifetime: self.lifetime,
            qr_image_template: self.qr_image_template,
            hash,
        })
    }
}

impl Default for GenerateQrRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateQrResponse {
    pub status: ApiStatus,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "qrString")]
    pub qr_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "qrImage")]
    pub qr_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abapay_deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_store: Option<String>,
    pub currency: String,
}
