//! QR API Operations

use crate::client::PayWayClient;
use crate::constants::endpoint;
use crate::error::Result;
use crate::types::{GenerateQrRequest, GenerateQrResponse};

impl PayWayClient {
    pub async fn generate_qr(&self, request: GenerateQrRequest) -> Result<GenerateQrResponse> {
        self.post(endpoint::GENERATE_QR, &request).await
    }
}
