//! RSA encryption utilities for PayWay payout operations

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rsa::{
    pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, traits::PublicKeyParts, Pkcs1v15Encrypt,
};

use crate::error::{PayWayError, Result};

const RSA_CHUNK_SIZE: usize = 117;
const RSA_CHUNK_SIZE_2048: usize = 245;

#[derive(Debug, Clone)]
pub struct RsaEncryptionResult {
    pub encrypted: String,
}

pub fn rsa_encrypt(data: &str, public_key: &str) -> Result<RsaEncryptionResult> {
    let public_key = parse_public_key(public_key)?;
    let key_size_bytes = public_key.size();
    let chunk_size = match key_size_bytes {
        128 => RSA_CHUNK_SIZE,
        256 => RSA_CHUNK_SIZE_2048,
        _ if key_size_bytes < 128 => RSA_CHUNK_SIZE,
        _ => RSA_CHUNK_SIZE_2048,
    };

    let mut encrypted_chunks = Vec::new();
    let mut rng = rand::thread_rng();

    for chunk in data.as_bytes().chunks(chunk_size) {
        let encrypted_chunk = public_key
            .encrypt(&mut rng, Pkcs1v15Encrypt, chunk)
            .map_err(|e| PayWayError::RsaError(format!("Encryption failed: {}", e)))?;
        encrypted_chunks.push(encrypted_chunk);
    }

    let combined = encrypted_chunks.concat();
    Ok(RsaEncryptionResult {
        encrypted: BASE64.encode(combined),
    })
}

fn parse_public_key(public_key_str: &str) -> Result<rsa::RsaPublicKey> {
    if public_key_str.contains("-----BEGIN") {
        if public_key_str.contains("PUBLIC KEY") {
            rsa::RsaPublicKey::from_public_key_pem(public_key_str)
                .map_err(|e| PayWayError::RsaError(format!("Failed to parse public key: {}", e)))
        } else if public_key_str.contains("RSA PUBLIC KEY") {
            rsa::RsaPublicKey::from_pkcs1_pem(public_key_str)
                .map_err(|e| PayWayError::RsaError(format!("Failed to parse public key: {}", e)))
        } else {
            Err(PayWayError::RsaError("Unknown PEM format".to_string()))
        }
    } else {
        Err(PayWayError::RsaError(
            "PEM format required. Provide key with -----BEGIN----- header.".to_string(),
        ))
    }
}

pub fn encrypt_merchant_auth(mc_id: &str, payee: &str, public_key: &str) -> Result<String> {
    let data = serde_json::json!({
        "mc_id": mc_id,
        "payee": payee
    });
    let json = serde_json::to_string(&data)
        .map_err(|e| PayWayError::RsaError(format!("JSON serialization failed: {}", e)))?;
    rsa_encrypt(&json, public_key).map(|r| r.encrypted)
}

pub fn encrypt_refund_auth(
    mc_id: &str,
    tran_id: &str,
    refund_amount: f64,
    public_key: &str,
) -> Result<String> {
    let data = serde_json::json!({
        "mc_id": mc_id,
        "tran_id": tran_id,
        "refund_amount": refund_amount
    });
    let json = serde_json::to_string(&data)
        .map_err(|e| PayWayError::RsaError(format!("JSON serialization failed: {}", e)))?;
    rsa_encrypt(&json, public_key).map(|r| r.encrypted)
}

pub fn encrypt_preauth_auth(
    mc_id: &str,
    tran_id: &str,
    complete_amount: f64,
    public_key: &str,
) -> Result<String> {
    let data = serde_json::json!({
        "mc_id": mc_id,
        "tran_id": tran_id,
        "complete_amount": complete_amount
    });
    let json = serde_json::to_string(&data)
        .map_err(|e| PayWayError::RsaError(format!("JSON serialization failed: {}", e)))?;
    rsa_encrypt(&json, public_key).map(|r| r.encrypted)
}
