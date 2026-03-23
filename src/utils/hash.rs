//! Hash generation utilities for PayWay API signatures

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hmac::{Hmac, Mac};
use sha2::Sha512;

type HmacSha512 = Hmac<Sha512>;

/// Generate HMAC-SHA512 hash for PayWay API requests
///
/// This function generates the hash signature required by PayWay API.
/// The hash is computed by concatenating the provided fields in order
/// and then computing HMAC-SHA512 with the API key.
///
/// # Arguments
///
/// * `api_key` - The API key provided by ABA Bank
/// * `fields` - Fields to concatenate in order (must be in correct order per API)
///
/// # Example
///
/// ```rust
/// use payway::utils::hash::generate_hash;
///
/// let api_key = "your_api_key";
/// let fields = vec!["20230101120000", "merchant123", "order001", "100.00"];
/// let hash = generate_hash(api_key, &fields);
/// ```
pub fn generate_hash(api_key: &str, fields: &[impl AsRef<str>]) -> String {
    let concatenated = fields
        .iter()
        .map(|f| f.as_ref())
        .collect::<Vec<&str>>()
        .join("");

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(concatenated.as_bytes());

    BASE64.encode(mac.finalize().into_bytes())
}

/// Generate hash for refund API
///
/// Refund API uses a different hash format that includes merchant_auth
pub fn generate_hash_for_refund(api_key: &str, request_time: &str, merchant_auth: &str) -> String {
    let concatenated = format!("{}{}", request_time, merchant_auth);

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(concatenated.as_bytes());

    BASE64.encode(mac.finalize().into_bytes())
}

/// Generate hash for payout API
///
/// Payout API uses a different hash format
pub fn generate_hash_for_payout(
    api_key: &str,
    req_time: &str,
    merchant_id: &str,
    tran_id: &str,
    beneficiaries: &str,
    amount: &str,
    custom_fields: &str,
    currency: &str,
) -> String {
    let concatenated = format!(
        "{}{}{}{}{}{}{}",
        req_time, merchant_id, tran_id, beneficiaries, amount, custom_fields, currency
    );

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(concatenated.as_bytes());

    BASE64.encode(mac.finalize().into_bytes())
}

/// Generate hash for beneficiary API
pub fn generate_hash_for_beneficiary(
    api_key: &str,
    request_time: &str,
    merchant_auth: &str,
) -> String {
    let concatenated = format!("{}{}", request_time, merchant_auth);

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(concatenated.as_bytes());

    BASE64.encode(mac.finalize().into_bytes())
}

/// Generate hash for pre-auth completion API
pub fn generate_hash_for_preauth(
    api_key: &str,
    merchant_id: &str,
    request_time: &str,
    merchant_auth: &str,
) -> String {
    let concatenated = format!("{}{}{}", merchant_id, request_time, merchant_auth);

    let mut mac =
        HmacSha512::new_from_slice(api_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(concatenated.as_bytes());

    BASE64.encode(mac.finalize().into_bytes())
}

/// Encode string to base64
pub fn encode_base64(data: impl AsRef<str>) -> String {
    BASE64.encode(data.as_ref().as_bytes())
}

/// Decode base64 string
pub fn decode_base64(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64.decode(data)
}

/// Encode JSON to base64
pub fn encode_json_base64<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string(value)?;
    Ok(encode_base64(json))
}

/// Encode optional JSON to base64
pub fn encode_optional_json_base64<T: serde::Serialize>(value: Option<&T>) -> Option<String> {
    value.and_then(|v| encode_json_base64(v).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[test]
    fn test_generate_hash() {
        let api_key = "test_api_key";
        let fields = vec!["20230101120000", "merchant123", "order001"];
        let hash = generate_hash(api_key, &fields);

        // Hash should be a valid base64 string
        assert!(!hash.is_empty());
        // Decoding should work
        assert!(BASE64.decode(&hash).is_ok());
    }

    #[test]
    fn test_generate_hash_consistency() {
        let api_key = "test_api_key";
        let fields1 = vec!["a", "b", "c"];
        let fields2 = vec!["a", "b", "c"];

        assert_eq!(
            generate_hash(api_key, &fields1),
            generate_hash(api_key, &fields2)
        );
    }

    #[test]
    fn test_generate_hash_different_order() {
        let api_key = "test_api_key";
        let fields1 = vec!["a", "b", "c"];
        let fields2 = vec!["c", "b", "a"];

        assert_ne!(
            generate_hash(api_key, &fields1),
            generate_hash(api_key, &fields2)
        );
    }

    #[test]
    fn test_encode_base64() {
        assert_eq!(encode_base64("hello"), "aGVsbG8=");
        assert_eq!(encode_base64(""), "");
    }

    #[test]
    fn test_decode_base64() {
        assert_eq!(decode_base64("aGVsbG8=").unwrap(), b"hello");
        assert!(decode_base64("invalid!").is_err());
    }

    #[test]
    fn test_encode_json_base64() {
        #[derive(Serialize)]
        struct TestStruct {
            name: String,
            value: i32,
        }

        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let encoded = encode_json_base64(&data).unwrap();
        assert!(!encoded.is_empty());

        // Verify it can be decoded
        let decoded = decode_base64(&encoded).unwrap();
        let json_str = String::from_utf8(decoded).unwrap();
        assert!(json_str.contains("\"name\":\"test\""));
    }
}
