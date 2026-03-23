//! Credentials on File Types

use serde::{Deserialize, Serialize};

use super::ApiStatus;
use crate::client::PayWayClient;
use crate::utils::hash::{encode_base64, encode_json_base64, generate_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAccountRequest {
    pub req_time: String,
    pub merchant_id: String,
    pub return_param: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_deeplink: Option<String>,
    pub hash: String,
}

impl LinkAccountRequest {
    pub fn new(client: &PayWayClient, return_param: impl Into<String>) -> Self {
        let req_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let return_param = return_param.into();

        let fields_for_hash = vec![&merchant_id, &req_time, ""];

        let hash = generate_hash(client.api_key(), &fields_for_hash);

        Self {
            req_time,
            merchant_id,
            return_param,
            return_url: None,
            return_deeplink: None,
            hash,
        }
    }

    pub fn with_return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(encode_base64(url.into()));
        self
    }

    pub fn with_return_deeplink(
        mut self,
        deeplink: serde_json::Value,
    ) -> Result<Self, crate::error::PayWayError> {
        self.return_deeplink = Some(encode_json_base64(&deeplink)?);
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkAccountResponse {
    pub deeplink: Option<String>,
    pub qr_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_image: Option<String>,
    pub expire_in: i64,
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseTokenRequest {
    pub merchant_id: String,
    pub tran_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwt: Option<String>,
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
    pub items: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_params: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<String>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<f64>,
    pub req_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub hash: String,
}

impl PurchaseTokenRequest {
    pub fn builder() -> PurchaseTokenRequestBuilder {
        PurchaseTokenRequestBuilder::new()
    }
}

pub struct PurchaseTokenRequestBuilder {
    merchant_id: String,
    tran_id: String,
    ctid: Option<String>,
    pwt: Option<String>,
    firstname: String,
    lastname: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    transaction_type: Option<String>,
    items: Option<Vec<serde_json::Value>>,
    return_url: Option<String>,
    custom_fields: Option<serde_json::Value>,
    return_params: Option<String>,
    payout: Option<Vec<serde_json::Value>>,
    amount: f64,
    shipping: Option<f64>,
    req_time: String,
    currency: Option<String>,
}

impl PurchaseTokenRequestBuilder {
    pub fn new() -> Self {
        Self {
            merchant_id: String::new(),
            tran_id: String::new(),
            ctid: None,
            pwt: None,
            firstname: String::new(),
            lastname: None,
            email: None,
            phone: None,
            transaction_type: None,
            items: None,
            return_url: None,
            custom_fields: None,
            return_params: None,
            payout: None,
            amount: 0.0,
            shipping: None,
            req_time: PayWayClient::generate_request_time(),
            currency: None,
        }
    }

    pub fn ctid(mut self, ctid: impl Into<String>) -> Self {
        self.ctid = Some(ctid.into());
        self
    }

    pub fn token(mut self, ctid: impl Into<String>, pwt: impl Into<String>) -> Self {
        self.ctid = Some(ctid.into());
        self.pwt = Some(pwt.into());
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.tran_id = id.into();
        self
    }

    pub fn first_name(mut self, name: impl Into<String>) -> Self {
        self.firstname = name.into();
        self
    }

    pub fn last_name(mut self, name: impl Into<String>) -> Self {
        self.lastname = Some(name.into());
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

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn build_with_client(
        self,
        client: &PayWayClient,
    ) -> Result<PurchaseTokenRequest, crate::error::PayWayError> {
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
            Some(f) => Some(encode_json_base64(f)?),
            None => None,
        };

        let amount_str = self.amount.to_string();
        let shipping_str = self
            .shipping
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
            self.ctid.as_deref().unwrap_or(""),
            self.pwt.as_deref().unwrap_or(""),
            &self.firstname,
            self.lastname.as_deref().unwrap_or(""),
            self.email.as_deref().unwrap_or(""),
            self.phone.as_deref().unwrap_or(""),
            self.transaction_type.as_deref().unwrap_or(""),
            self.return_url.as_deref().unwrap_or(""),
            self.currency.as_deref().unwrap_or(""),
            custom_fields_b64.as_deref().unwrap_or(""),
            self.return_params.as_deref().unwrap_or(""),
            payout_b64.as_deref().unwrap_or(""),
        ];

        let hash = generate_hash(api_key, &fields_for_hash);

        Ok(PurchaseTokenRequest {
            merchant_id,
            tran_id: self.tran_id,
            ctid: self.ctid,
            pwt: self.pwt,
            firstname: self.firstname,
            lastname: self.lastname,
            email: self.email,
            phone: self.phone,
            transaction_type: self.transaction_type,
            items: items_b64,
            return_url: self.return_url,
            custom_fields: custom_fields_b64,
            return_params: self.return_params,
            payout: payout_b64,
            amount: self.amount,
            shipping: self.shipping,
            req_time: self.req_time,
            currency: self.currency,
            hash,
        })
    }
}

impl Default for PurchaseTokenRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseTokenResponse {
    pub tran_id: String,
    pub payment_status: PaymentStatusResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentStatusResponse {
    pub status: String,
    pub code: String,
    pub description: String,
    pub pw_tran_id: String,
}
