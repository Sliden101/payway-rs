//! Payout API Operations

use crate::client::PayWayClient;
use crate::constants::endpoint;
use crate::error::Result;
use crate::types::{
    AddBeneficiaryRequest, AddBeneficiaryResponse, PayoutBeneficiary,
    PayoutRequest as PayoutApiRequest, PayoutResponse, UpdateBeneficiaryStatusRequest,
    UpdateBeneficiaryStatusResponse,
};

impl PayWayClient {
    pub async fn add_beneficiary(
        &self,
        payee: impl Into<String>,
    ) -> Result<AddBeneficiaryResponse> {
        let request = AddBeneficiaryRequest::new(self, payee).await?;
        self.post(endpoint::ADD_BENEFICIARY, &request).await
    }

    pub async fn update_beneficiary_status(
        &self,
        payee: impl Into<String>,
        enable: bool,
    ) -> Result<UpdateBeneficiaryStatusResponse> {
        let request = UpdateBeneficiaryStatusRequest::new(self, payee, enable).await?;
        self.post(endpoint::UPDATE_BENEFICIARY, &request).await
    }

    pub async fn payout(
        &self,
        tran_id: impl Into<String>,
        beneficiaries: Vec<PayoutBeneficiary>,
        amount: f64,
        currency: impl Into<String>,
    ) -> Result<PayoutResponse> {
        let request = PayoutApiRequest::new(self, tran_id, beneficiaries, amount, currency).await?;
        self.post(endpoint::PAYOUT, &request).await
    }
}
