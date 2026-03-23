//! Pre-auth API Operations

use crate::client::PayWayClient;
use crate::constants::endpoint;
use crate::error::Result;
use crate::types::{CompletePreAuthRequest, CompletePreAuthResponse};

impl PayWayClient {
    pub async fn complete_preauth(
        &self,
        tran_id: impl Into<String>,
        complete_amount: f64,
    ) -> Result<CompletePreAuthResponse> {
        let request = CompletePreAuthRequest::new(self, tran_id, complete_amount).await?;
        self.post(endpoint::PRE_AUTH_COMPLETE, &request).await
    }
}
