//! Credentials on File API Operations

use crate::client::PayWayClient;
use crate::constants::endpoint;
use crate::error::Result;
use crate::types::{
    LinkAccountRequest, LinkAccountResponse, PurchaseTokenRequest, PurchaseTokenResponse,
};

impl PayWayClient {
    pub async fn link_account(
        &self,
        return_param: impl Into<String>,
    ) -> Result<LinkAccountResponse> {
        let request = LinkAccountRequest::new(self, return_param);
        self.post(endpoint::LINK_ACCOUNT, &request).await
    }

    pub async fn purchase_with_token(
        &self,
        request: PurchaseTokenRequest,
    ) -> Result<PurchaseTokenResponse> {
        self.post(endpoint::PURCHASE_TOKEN, &request).await
    }
}
