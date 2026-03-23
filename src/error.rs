//! PayWay Error Types

use thiserror::Error;

/// Result type alias for PayWay operations
pub type Result<T> = std::result::Result<T, PayWayError>;

/// PayWay SDK Error Types
#[derive(Error, Debug)]
pub enum PayWayError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    /// Hash generation failed
    #[error("Hash generation failed: {0}")]
    HashError(String),

    /// RSA encryption failed
    #[error("RSA encryption failed: {0}")]
    RsaError(String),

    /// Base64 encoding/decoding error
    #[error("Base64 error: {0}")]
    Base64Error(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// HTTP request error
    #[error("HTTP error: {0}")]
    HttpError(String),

    /// API returned an error
    #[error("API error: code={code}, message={message}")]
    ApiError { code: String, message: String },

    /// Transaction not found
    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    /// Duplicated transaction
    #[error("Duplicated transaction ID: {0}")]
    DuplicatedTransaction(String),

    /// Invalid hash
    #[error("Invalid hash signature")]
    InvalidHash,

    /// Rate limit exceeded
    #[error("Rate limit exceeded, please try again later")]
    RateLimitExceeded,

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Unexpected response format
    #[error("Unexpected response format: {0}")]
    UnexpectedResponse(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<reqwest::Error> for PayWayError {
    fn from(err: reqwest::Error) -> Self {
        PayWayError::HttpError(err.to_string())
    }
}

impl PayWayError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            PayWayError::RateLimitExceeded => true,
            PayWayError::NetworkError(_) => true,
            PayWayError::HttpError(ref msg) => {
                msg.contains("429")
                    || msg.contains("500")
                    || msg.contains("502")
                    || msg.contains("503")
            }
            _ => false,
        }
    }

    /// Get error code if it's an API error
    pub fn api_code(&self) -> Option<&str> {
        match self {
            PayWayError::ApiError { code, .. } => Some(code),
            _ => None,
        }
    }
}

/// Error codes from PayWay API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ErrorCode {
    Success = 0,
    WrongHash = 1,
    InvalidTransactionId = 2,
    InvalidAmount = 3,
    DuplicatedTransaction = 4,
    TransactionNotFound = 5,
    WrongDomain = 6,
    WrongReturnParam = 7,
    DataSaveError = 8,
    WrongShippingPrice = 10,
    InternalError = 11,
    CurrencyNotAllowed = 12,
    InvalidItems = 13,
    InvalidCreditMultiAcc = 14,
    InvalidChannel = 15,
    InvalidFirstName = 16,
    InvalidLastName = 17,
    InvalidPhone = 18,
    InvalidEmail = 19,
    ContactMerchant = 20,
    ApiLifetimeEnd = 21,
    PreAuthNotEnabled = 22,
    PaymentOptionNotEnabled = 23,
    CannotDecrypt = 24,
    MaxPayoutExceeded = 25,
    InvalidMerchantProfile = 26,
    InvalidCtid = 27,
    InvalidPwt = 28,
    InvalidCtidOrPwt = 29,
    MerchantNotEnabledCof = 30,
    Unsecure3ds = 31,
    CannotIdentifyCardOrigin = 33,
    InvalidExchangeRate = 34,
    PayoutInfoInvalid = 35,
    PayoutAccountInvalid = 36,
    PayoutNotInWhitelist = 37,
    PayoutInvalidTranId = 38,
    PayoutDuplicatedAccount = 39,
    PayoutDuplicatedTranId = 40,
    PayoutMidNotFound = 41,
    PayoutAccountInvalidStatus = 42,
    MerchantMidMissing = 43,
    TransactionLimitReached = 44,
    ZeroAmountNotAllowed = 45,
    KhrDecimalNotAllowed = 46,
    KhrMustBeGreater100 = 47,
    InvalidParameters = 48,
    InvalidStartDate = 49,
    InvalidEndDate = 50,
    InvalidDateRange = 51,
    MaxDateRange3Days = 52,
    InvalidAmountRange = 53,
    TransactionExpired = 54,
    WechatQrError = 55,
    WechatValidationError = 56,
    CannotIdentifyCardSource = 57,
    InvalidCardNumber = 58,
    PayoutMidAccountMismatch = 59,
    QrStringError = 60,
    SomethingWentWrong = 61,
    QrAlreadyUsed = 62,
    TransactionExistsInCore = 63,
    PayerSameAsMerchant = 64,
    MerchantMidNotFoundInCore = 65,
    QrOnInvoiceNotAvailable = 66,
    TransactionExpiredRetry = 67,
    LifetimeLessThan3Mins = 68,
    DailyLimitReached = 70,
    CardPayoutNotAllowed = 71,
    SettlementAccountClosed = 72,
    InvalidTransactionStatus = 73,
    InvalidTranIdOrMerchantId = 74,
    TranIdNotFound = 75,
    InvalidAdditionalParams = 76,
    TransactionFeesNotSupported = 77,
    CardDiscountIncompatible = 78,
    GooglePayTokenMissing = 79,
    GooglePayDecryptFailed = 80,
    ReturnUrlNotInWhitelist = 81,
    PayoutAmountExceeded = 82,
    CredentialDisabled = 83,
    CredentialExpired = 84,
    TransactionAmountLimit = 85,
    UnsupportedPurchaseMode = 86,
    CredentialRemoved = 87,
    PaymentCancelled = 200,
    PaymentDeclined = 201,
    Unauthorized = 401,
    Forbidden = 403,
    TooManyRequests = 429,
    SystemMaintenance = 503,
}

impl ErrorCode {
    pub fn from_code(code: &str) -> Option<Self> {
        let code_num = code.parse::<u32>().ok()?;
        match code_num {
            0 => Some(Self::Success),
            1 => Some(Self::WrongHash),
            2 => Some(Self::InvalidTransactionId),
            3 => Some(Self::InvalidAmount),
            4 => Some(Self::DuplicatedTransaction),
            5 => Some(Self::TransactionNotFound),
            6 => Some(Self::WrongDomain),
            7 => Some(Self::WrongReturnParam),
            8 => Some(Self::DataSaveError),
            10 => Some(Self::WrongShippingPrice),
            11 => Some(Self::InternalError),
            12 => Some(Self::CurrencyNotAllowed),
            13 => Some(Self::InvalidItems),
            14 => Some(Self::InvalidCreditMultiAcc),
            15 => Some(Self::InvalidChannel),
            16 => Some(Self::InvalidFirstName),
            17 => Some(Self::InvalidLastName),
            18 => Some(Self::InvalidPhone),
            19 => Some(Self::InvalidEmail),
            20 => Some(Self::ContactMerchant),
            21 => Some(Self::ApiLifetimeEnd),
            22 => Some(Self::PreAuthNotEnabled),
            23 => Some(Self::PaymentOptionNotEnabled),
            24 => Some(Self::CannotDecrypt),
            25 => Some(Self::MaxPayoutExceeded),
            26 => Some(Self::InvalidMerchantProfile),
            27 => Some(Self::InvalidCtid),
            28 => Some(Self::InvalidPwt),
            29 => Some(Self::InvalidCtidOrPwt),
            30 => Some(Self::MerchantNotEnabledCof),
            31 => Some(Self::Unsecure3ds),
            33 => Some(Self::CannotIdentifyCardOrigin),
            34 => Some(Self::InvalidExchangeRate),
            35 => Some(Self::PayoutInfoInvalid),
            36 => Some(Self::PayoutAccountInvalid),
            37 => Some(Self::PayoutNotInWhitelist),
            38 => Some(Self::PayoutInvalidTranId),
            39 => Some(Self::PayoutDuplicatedAccount),
            40 => Some(Self::PayoutDuplicatedTranId),
            41 => Some(Self::PayoutMidNotFound),
            42 => Some(Self::PayoutAccountInvalidStatus),
            43 => Some(Self::MerchantMidMissing),
            44 => Some(Self::TransactionLimitReached),
            45 => Some(Self::ZeroAmountNotAllowed),
            46 => Some(Self::KhrDecimalNotAllowed),
            47 => Some(Self::KhrMustBeGreater100),
            48 => Some(Self::InvalidParameters),
            49 => Some(Self::InvalidStartDate),
            50 => Some(Self::InvalidEndDate),
            51 => Some(Self::InvalidDateRange),
            52 => Some(Self::MaxDateRange3Days),
            53 => Some(Self::InvalidAmountRange),
            54 => Some(Self::TransactionExpired),
            55 => Some(Self::WechatQrError),
            56 => Some(Self::WechatValidationError),
            57 => Some(Self::CannotIdentifyCardSource),
            58 => Some(Self::InvalidCardNumber),
            59 => Some(Self::PayoutMidAccountMismatch),
            60 => Some(Self::QrStringError),
            61 => Some(Self::SomethingWentWrong),
            62 => Some(Self::QrAlreadyUsed),
            63 => Some(Self::TransactionExistsInCore),
            64 => Some(Self::PayerSameAsMerchant),
            65 => Some(Self::MerchantMidNotFoundInCore),
            66 => Some(Self::QrOnInvoiceNotAvailable),
            67 => Some(Self::TransactionExpiredRetry),
            68 => Some(Self::LifetimeLessThan3Mins),
            70 => Some(Self::DailyLimitReached),
            71 => Some(Self::CardPayoutNotAllowed),
            72 => Some(Self::SettlementAccountClosed),
            73 => Some(Self::InvalidTransactionStatus),
            74 => Some(Self::InvalidTranIdOrMerchantId),
            75 => Some(Self::TranIdNotFound),
            76 => Some(Self::InvalidAdditionalParams),
            77 => Some(Self::TransactionFeesNotSupported),
            78 => Some(Self::CardDiscountIncompatible),
            79 => Some(Self::GooglePayTokenMissing),
            80 => Some(Self::GooglePayDecryptFailed),
            81 => Some(Self::ReturnUrlNotInWhitelist),
            82 => Some(Self::PayoutAmountExceeded),
            83 => Some(Self::CredentialDisabled),
            84 => Some(Self::CredentialExpired),
            85 => Some(Self::TransactionAmountLimit),
            86 => Some(Self::UnsupportedPurchaseMode),
            87 => Some(Self::CredentialRemoved),
            200 => Some(Self::PaymentCancelled),
            201 => Some(Self::PaymentDeclined),
            401 => Some(Self::Unauthorized),
            403 => Some(Self::Forbidden),
            429 => Some(Self::TooManyRequests),
            503 => Some(Self::SystemMaintenance),
            _ => None,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::WrongHash => "Wrong hash signature",
            Self::InvalidTransactionId => "Invalid transaction ID",
            Self::InvalidAmount => "Invalid transaction amount",
            Self::DuplicatedTransaction => "Duplicated transaction ID",
            Self::TransactionNotFound => "Transaction not found",
            Self::WrongDomain => "Requested domain is not in whitelist",
            Self::WrongReturnParam => "Wrong return parameter",
            Self::DataSaveError => "Something went wrong while saving data",
            Self::WrongShippingPrice => "Wrong shipping price",
            Self::InternalError => "Something went wrong. Try again or contact merchant",
            Self::CurrencyNotAllowed => "Payment currency is not allowed",
            Self::InvalidItems => "Invalid items",
            Self::InvalidCreditMultiAcc => "Invalid credit multi account",
            Self::InvalidChannel => "Invalid or missing channel values",
            Self::InvalidFirstName => "Invalid first name",
            Self::InvalidLastName => "Invalid last name",
            Self::InvalidPhone => "Invalid phone number",
            Self::InvalidEmail => "Invalid email",
            Self::ContactMerchant => "Please contact merchant",
            Self::ApiLifetimeEnd => "End of API lifetime",
            Self::PreAuthNotEnabled => "Pre-auth transaction is not enabled",
            Self::PaymentOptionNotEnabled => "Selected payment option is not enabled",
            Self::CannotDecrypt => "Cannot decrypt data",
            Self::MaxPayoutExceeded => "Maximum 10 payout per request",
            Self::InvalidMerchantProfile => "Invalid merchant profile",
            Self::InvalidCtid => "Invalid consumer token ID",
            Self::InvalidPwt => "Invalid PayWay token",
            Self::InvalidCtidOrPwt => "Invalid consumer token or PayWay token",
            Self::MerchantNotEnabledCof => "Merchant is not enabled for COF",
            Self::Unsecure3ds => "Unsecure 3DS page",
            Self::CannotIdentifyCardOrigin => "Cannot identify card origin",
            Self::InvalidExchangeRate => "Exchange rate data is invalid",
            Self::PayoutInfoInvalid => "Payout info is invalid",
            Self::PayoutAccountInvalid => "Payout account or amount is invalid",
            Self::PayoutNotInWhitelist => "Payout accounts are not in whitelist",
            Self::PayoutInvalidTranId => "Payout contains invalid transaction ID",
            Self::PayoutDuplicatedAccount => "Payout contains duplicated account",
            Self::PayoutDuplicatedTranId => "Payout contains duplicated transaction ID",
            Self::PayoutMidNotFound => "Payout info contains MID not linked to any merchant",
            Self::PayoutAccountInvalidStatus => "Payout info contains account with invalid status",
            Self::MerchantMidMissing => "Merchant profile's MID is missing",
            Self::TransactionLimitReached => "Purchase amount has reached transaction limit",
            Self::ZeroAmountNotAllowed => "Purchase with zero amount is not allowed",
            Self::KhrDecimalNotAllowed => "Purchase amount for KHR cannot contain decimals",
            Self::KhrMustBeGreater100 => "KHR amount must be greater than 100 KHR",
            Self::InvalidParameters => "Invalid requested parameters",
            Self::InvalidStartDate => "Invalid start date",
            Self::InvalidEndDate => "Invalid end date",
            Self::InvalidDateRange => "Invalid date range",
            Self::MaxDateRange3Days => "Maximum date range is 3 days",
            Self::InvalidAmountRange => "Invalid amount range",
            Self::TransactionExpired => "Transaction is expired",
            Self::WechatQrError => "Unable to request QR from WeChat",
            Self::WechatValidationError => "Unable to validate transaction with WeChat",
            Self::CannotIdentifyCardSource => "Unable to validate card source",
            Self::InvalidCardNumber => "Invalid card number",
            Self::PayoutMidAccountMismatch => "Payout info cannot match MID and ABA account",
            Self::QrStringError => "Something went wrong with QR string",
            Self::SomethingWentWrong => "Something went wrong",
            Self::QrAlreadyUsed => "QR is already in use",
            Self::TransactionExistsInCore => "Transaction already exists in core banking",
            Self::PayerSameAsMerchant => "Payer's account is same as merchant's account",
            Self::MerchantMidNotFoundInCore => "Merchant profile's MID not found in core banking",
            Self::QrOnInvoiceNotAvailable => "QR on invoice is not available for this profile",
            Self::TransactionExpiredRetry => "Transaction expired, please re-initiate",
            Self::LifetimeLessThan3Mins => "Transaction lifetime cannot be less than 3 minutes",
            Self::DailyLimitReached => "Total purchase amount has reached daily limit",
            Self::CardPayoutNotAllowed => "Payout for card payment is not allowed to ABA account",
            Self::SettlementAccountClosed => "Merchant's settlement account is closed",
            Self::InvalidTransactionStatus => "Invalid transaction status",
            Self::InvalidTranIdOrMerchantId => "Invalid transaction ID or merchant ID",
            Self::TranIdNotFound => "Transaction ID not found",
            Self::InvalidAdditionalParams => "Invalid additional parameters",
            Self::TransactionFeesNotSupported => "Merchant transactions do not support fees",
            Self::CardDiscountIncompatible => "Card payment incompatible with discount program",
            Self::GooglePayTokenMissing => "Payment token missing in Google Pay",
            Self::GooglePayDecryptFailed => "Failed to decrypt Google Pay token",
            Self::ReturnUrlNotInWhitelist => "Return URL is not in whitelist",
            Self::PayoutAmountExceeded => "Payout exceeded maximum allowable amount",
            Self::CredentialDisabled => "Payment credential is disabled",
            Self::CredentialExpired => "Payment credential is expired",
            Self::TransactionAmountLimit => "Purchase reached limit amount per transaction",
            Self::UnsupportedPurchaseMode => "Unsupported merchant purchase mode",
            Self::CredentialRemoved => "Payment credential is removed",
            Self::PaymentCancelled => "Payment was cancelled",
            Self::PaymentDeclined => "Payment was declined",
            Self::Unauthorized => "Unauthorized access",
            Self::Forbidden => "Access forbidden",
            Self::TooManyRequests => "Too many requests, please try again later",
            Self::SystemMaintenance => "System under maintenance",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_from_code() {
        assert_eq!(ErrorCode::from_code("0"), Some(ErrorCode::Success));
        assert_eq!(ErrorCode::from_code("00"), Some(ErrorCode::Success));
        assert_eq!(ErrorCode::from_code("1"), Some(ErrorCode::WrongHash));
        assert_eq!(
            ErrorCode::from_code("4"),
            Some(ErrorCode::DuplicatedTransaction)
        );
        assert_eq!(
            ErrorCode::from_code("429"),
            Some(ErrorCode::TooManyRequests)
        );
        assert_eq!(ErrorCode::from_code("999"), None);
    }

    #[test]
    fn test_error_code_description() {
        assert_eq!(ErrorCode::Success.description(), "Success");
        assert_eq!(ErrorCode::WrongHash.description(), "Wrong hash signature");
        assert_eq!(
            ErrorCode::TransactionNotFound.description(),
            "Transaction not found"
        );
    }

    #[test]
    fn test_payway_error_is_retryable() {
        assert!(PayWayError::RateLimitExceeded.is_retryable());
    }
}
