//! PayWay Client

use std::sync::Arc;

use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, error};

use crate::config::PayWayConfig;
use crate::error::{PayWayError, Result};

#[derive(Clone)]
pub struct PayWayClient {
    config: Arc<PayWayConfig>,
    http_client: Client,
}

impl PayWayClient {
    pub fn new(merchant_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self::with_config(PayWayConfig::sandbox(merchant_id, api_key))
    }

    pub fn with_config(config: PayWayConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            config: Arc::new(config),
            http_client,
        }
    }

    pub fn config(&self) -> &PayWayConfig {
        &self.config
    }

    pub fn merchant_id(&self) -> &str {
        &self.config.merchant_id
    }

    pub fn api_key(&self) -> &str {
        &self.config.api_key
    }

    pub fn environment(&self) -> crate::config::Environment {
        self.config.environment
    }

    pub fn base_url(&self) -> &'static str {
        self.config.base_url()
    }

    pub fn is_sandbox(&self) -> bool {
        self.config.is_sandbox()
    }

    pub fn has_rsa_keys(&self) -> bool {
        self.config.has_rsa_keys()
    }

    pub fn rsa_public_key(&self) -> Option<&str> {
        self.config.rsa_public_key.as_deref()
    }

    pub async fn post<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url(), endpoint);

        debug!("POST {}", url);

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        debug!("Response status: {}, body: {}", status, response_text);

        if !status.is_success() {
            error!("HTTP error: {} - {}", status, response_text);
            return Err(PayWayError::HttpError(format!(
                "{}: {}",
                status, response_text
            )));
        }

        serde_json::from_str(&response_text).map_err(|e| PayWayError::JsonError(e))
    }

    pub async fn post_form<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url(), endpoint);

        debug!("POST (form) {}", url);

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(body)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        debug!("Response status: {}, body: {}", status, response_text);

        if !status.is_success() {
            error!("HTTP error: {} - {}", status, response_text);
            return Err(PayWayError::HttpError(format!(
                "{}: {}",
                status, response_text
            )));
        }

        serde_json::from_str(&response_text).map_err(|e| PayWayError::JsonError(e))
    }

    pub fn generate_request_time() -> String {
        chrono::Utc::now().format("%Y%m%d%H%M%S").to_string()
    }
}

impl std::fmt::Debug for PayWayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PayWayClient")
            .field("merchant_id", &self.config.merchant_id)
            .field("environment", &self.config.environment)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_request_time() {
        let time = PayWayClient::generate_request_time();
        assert_eq!(time.len(), 14);
        assert!(time.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_new_client() {
        let client = PayWayClient::new("test_merchant", "test_api_key");
        assert_eq!(client.merchant_id(), "test_merchant");
        assert_eq!(client.api_key(), "test_api_key");
        assert!(client.is_sandbox());
    }
}
