//! Payout Types

use serde::{Deserialize, Serialize};

use super::ApiStatus;
use crate::client::PayWayClient;
use crate::error::Result;
use crate::utils::hash::{generate_hash_for_payout, generate_hash_for_refund};
use crate::utils::rsa;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBeneficiaryRequest {
    pub request_time: String,
    pub merchant_id: String,
    pub merchant_auth: String,
    pub hash: String,
}

impl AddBeneficiaryRequest {
    pub async fn new(client: &PayWayClient, payee: impl Into<String>) -> Result<Self> {
        let request_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let payee = payee.into();

        let public_key = client.rsa_public_key().ok_or_else(|| {
            crate::error::PayWayError::Config(
                "RSA public key not configured. Required for payout operations.".to_string(),
            )
        })?;

        let merchant_auth = rsa::encrypt_merchant_auth(&merchant_id, &payee, public_key)?;

        let hash = generate_hash_for_refund(client.api_key(), &request_time, &merchant_auth);

        Ok(Self {
            request_time,
            merchant_id,
            merchant_auth,
            hash,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBeneficiaryResponse {
    pub data: Option<BeneficiaryData>,
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficiaryData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub payee: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub status: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutRequest {
    pub request_time: String,
    pub merchant_id: String,
    pub tran_id: String,
    pub beneficiaries: String,
    pub amount: f64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<String>,
    pub hash: String,
}

impl PayoutRequest {
    pub async fn new(
        client: &PayWayClient,
        tran_id: impl Into<String>,
        beneficiaries: Vec<PayoutBeneficiary>,
        amount: f64,
        currency: impl Into<String>,
    ) -> Result<Self> {
        let request_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let tran_id = tran_id.into();
        let currency = currency.into();

        let public_key = client.rsa_public_key().ok_or_else(|| {
            crate::error::PayWayError::Config(
                "RSA public key not configured. Required for payout operations.".to_string(),
            )
        })?;

        let beneficiaries_json = serde_json::to_string(&beneficiaries)?;
        let beneficiaries = rsa::rsa_encrypt(&beneficiaries_json, public_key)?.encrypted;

        let custom_fields = "{}".to_string();

        let hash = generate_hash_for_payout(
            client.api_key(),
            &request_time,
            &merchant_id,
            &tran_id,
            &beneficiaries,
            &amount.to_string(),
            &custom_fields,
            &currency,
        );

        Ok(Self {
            request_time,
            merchant_id,
            tran_id,
            beneficiaries,
            amount,
            currency,
            custom_fields: Some(custom_fields),
            hash,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutBeneficiary {
    pub account: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutResponse {
    pub transaction_id: String,
    pub transaction_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apv: Option<String>,
    pub transaction_amount: f64,
    pub transaction_currency: String,
    pub beneficiaries: Vec<PayoutBeneficiaryResult>,
    pub status: ApiStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutBeneficiaryResult {
    pub payout_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub mid_acccount: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBeneficiaryStatusRequest {
    pub request_time: String,
    pub merchant_id: String,
    pub merchant_auth: String,
    pub status: i32,
    pub hash: String,
}

impl UpdateBeneficiaryStatusRequest {
    pub async fn new(
        client: &PayWayClient,
        payee: impl Into<String>,
        enable: bool,
    ) -> Result<Self> {
        let request_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let payee = payee.into();
        let status = if enable { 1 } else { 0 };

        let public_key = client.rsa_public_key().ok_or_else(|| {
            crate::error::PayWayError::Config(
                "RSA public key not configured. Required for payout operations.".to_string(),
            )
        })?;

        let merchant_auth = rsa::encrypt_merchant_auth(&merchant_id, &payee, public_key)?;

        let hash = generate_hash_for_refund(client.api_key(), &request_time, &merchant_auth);

        Ok(Self {
            request_time,
            merchant_id,
            merchant_auth,
            status,
            hash,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBeneficiaryStatusResponse {
    pub data: Option<BeneficiaryData>,
    pub status: ApiStatus,
}
