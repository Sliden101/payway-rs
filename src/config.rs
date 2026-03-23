//! PayWay Configuration

use serde::{Deserialize, Serialize};

use crate::constants::{BASE_URL_PROD, BASE_URL_SANDBOX};

/// PayWay Environment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Environment {
    /// Sandbox environment for testing
    #[default]
    Sandbox,
    /// Production environment
    Production,
}

impl Environment {
    /// Get the base URL for this environment
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Sandbox => BASE_URL_SANDBOX,
            Environment::Production => BASE_URL_PROD,
        }
    }

    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            Environment::Sandbox => "Sandbox",
            Environment::Production => "Production",
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// PayWay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayWayConfig {
    /// Merchant ID provided by ABA Bank
    pub merchant_id: String,
    /// API key provided by ABA Bank
    pub api_key: String,
    /// RSA private key for payout operations (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsa_private_key: Option<String>,
    /// RSA public key provided by ABA Bank for payout operations (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsa_public_key: Option<String>,
    /// Custom base URL for testing (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Environment (sandbox or production)
    #[serde(default)]
    pub environment: Environment,
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    /// Maximum retries for failed requests
    #[serde(default = "default_retries")]
    pub max_retries: u32,
}

fn default_timeout() -> u64 {
    30
}

fn default_retries() -> u32 {
    3
}

impl PayWayConfig {
    /// Create a new configuration for sandbox environment
    pub fn sandbox(merchant_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            merchant_id: merchant_id.into(),
            api_key: api_key.into(),
            rsa_private_key: None,
            rsa_public_key: None,
            base_url: None,
            environment: Environment::Sandbox,
            timeout_secs: default_timeout(),
            max_retries: default_retries(),
        }
    }

    /// Create a new configuration for production environment
    pub fn production(merchant_id: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            merchant_id: merchant_id.into(),
            api_key: api_key.into(),
            rsa_private_key: None,
            rsa_public_key: None,
            base_url: None,
            environment: Environment::Production,
            timeout_secs: default_timeout(),
            max_retries: default_retries(),
        }
    }

    /// Set RSA keys for payout operations
    pub fn with_rsa_keys(
        mut self,
        private_key: impl Into<String>,
        public_key: impl Into<String>,
    ) -> Self {
        self.rsa_private_key = Some(private_key.into());
        self.rsa_public_key = Some(public_key.into());
        self
    }

    /// Set custom base URL (useful for testing with WireMock)
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    /// Get the base URL
    pub fn base_url(&self) -> &'static str {
        self.environment.base_url()
    }

    /// Check if this is sandbox environment
    pub fn is_sandbox(&self) -> bool {
        self.environment == Environment::Sandbox
    }

    /// Check if RSA keys are configured for payouts
    pub fn has_rsa_keys(&self) -> bool {
        self.rsa_private_key.is_some() && self.rsa_public_key.is_some()
    }
}

impl From<(String, String)> for PayWayConfig {
    fn from((merchant_id, api_key): (String, String)) -> Self {
        Self::sandbox(merchant_id, api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_base_url() {
        assert_eq!(
            Environment::Sandbox.base_url(),
            "https://checkout-sandbox.payway.com.kh/"
        );
        assert_eq!(
            Environment::Production.base_url(),
            "https://checkout.payway.com.kh/"
        );
    }

    #[test]
    fn test_config_sandbox() {
        let config = PayWayConfig::sandbox("merchant_123", "api_key_456");
        assert_eq!(config.merchant_id, "merchant_123");
        assert_eq!(config.api_key, "api_key_456");
        assert!(config.is_sandbox());
    }

    #[test]
    fn test_config_production() {
        let config = PayWayConfig::production("merchant_123", "api_key_456");
        assert_eq!(config.merchant_id, "merchant_123");
        assert_eq!(config.api_key, "api_key_456");
        assert!(!config.is_sandbox());
    }

    #[test]
    fn test_config_with_rsa_keys() {
        let config = PayWayConfig::sandbox("merchant_123", "api_key_456")
            .with_rsa_keys("private_key", "public_key");
        assert!(config.has_rsa_keys());
    }
}
