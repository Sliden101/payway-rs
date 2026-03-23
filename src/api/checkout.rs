//! Checkout API Operations

use crate::client::PayWayClient;
use crate::constants::endpoint;
use crate::error::Result;
use crate::types::{
    CheckTransactionRequest, CheckTransactionResponse, CloseTransactionRequest,
    CloseTransactionResponse, PurchaseParams, PurchaseResponse, RefundRequest, RefundResponse,
    TransactionDetailRequest, TransactionDetailResponse,
};

impl PayWayClient {
    pub async fn create_transaction(&self, params: PurchaseParams) -> Result<PurchaseResponse> {
        self.post_form(endpoint::PURCHASE, &params).await
    }

    pub async fn check_transaction(
        &self,
        tran_id: impl Into<String>,
    ) -> Result<CheckTransactionResponse> {
        let request = CheckTransactionRequest::new(self, tran_id);
        self.post(endpoint::CHECK_TRANSACTION, &request).await
    }

    pub async fn get_transaction_detail(
        &self,
        tran_id: impl Into<String>,
    ) -> Result<TransactionDetailResponse> {
        let request = TransactionDetailRequest::new(self, tran_id);
        self.post(endpoint::TRANSACTION_DETAIL, &request).await
    }

    pub async fn close_transaction(
        &self,
        tran_id: impl Into<String>,
    ) -> Result<CloseTransactionResponse> {
        let request = CloseTransactionRequest::new(self, tran_id);
        self.post(endpoint::CLOSE_TRANSACTION, &request).await
    }

    pub async fn refund(
        &self,
        tran_id: impl Into<String>,
        refund_amount: f64,
    ) -> Result<RefundResponse> {
        let request_time = Self::generate_request_time();
        let merchant_id = self.merchant_id().to_string();
        let tran_id = tran_id.into();

        let public_key = self.rsa_public_key().ok_or_else(|| {
            crate::error::PayWayError::Config(
                "RSA public key not configured. Required for refund operations.".to_string(),
            )
        })?;

        let merchant_auth = crate::utils::rsa::encrypt_refund_auth(
            &merchant_id,
            &tran_id,
            refund_amount,
            public_key,
        )?;

        let hash = crate::utils::hash::generate_hash_for_refund(
            self.api_key(),
            &request_time,
            &merchant_auth,
        );

        let request = RefundRequest {
            request_time,
            merchant_id,
            merchant_auth,
            hash,
        };

        self.post(endpoint::REFUND, &request).await
    }
}
