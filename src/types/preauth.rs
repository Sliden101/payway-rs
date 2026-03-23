//! Pre-auth Types

use serde::{Deserialize, Serialize};

use super::ApiStatus;
use crate::client::PayWayClient;
use crate::error::Result;
use crate::utils::rsa;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletePreAuthRequest {
    pub request_time: String,
    pub merchant_id: String,
    pub merchant_auth: String,
    pub hash: String,
}

impl CompletePreAuthRequest {
    pub async fn new(
        client: &PayWayClient,
        tran_id: impl Into<String>,
        complete_amount: f64,
    ) -> Result<Self> {
        let request_time = PayWayClient::generate_request_time();
        let merchant_id = client.merchant_id().to_string();
        let tran_id = tran_id.into();

        let public_key = client.rsa_public_key().ok_or_else(|| {
            crate::error::PayWayError::Config(
                "RSA public key not configured. Required for pre-auth operations.".to_string(),
            )
        })?;

        let merchant_auth =
            rsa::encrypt_preauth_auth(&merchant_id, &tran_id, complete_amount, public_key)?;

        let hash = crate::utils::hash::generate_hash_for_preauth(
            client.api_key(),
            &merchant_id,
            &request_time,
            &merchant_auth,
        );

        Ok(Self {
            request_time,
            merchant_id,
            merchant_auth,
            hash,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletePreAuthResponse {
    pub grand_total: f64,
    pub currency: String,
    pub transaction_status: String,
    pub status: ApiStatus,
}
