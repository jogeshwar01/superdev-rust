use crate::models::{SignMessageRequest, VerifyMessageRequest};
use crate::utils::{error_response, success_response};
use actix_web::{web, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signature, Signer},
    signer::keypair::Keypair,
};

// Endpoint: POST /message/sign
pub async fn sign_message(req: web::Json<SignMessageRequest>) -> Result<HttpResponse> {
    // Check for missing required fields
    if req.message.is_empty() || req.secret.is_empty() {
        return Ok(error_response("Missing required fields"));
    }

    // Decode the secret key
    let secret_bytes = match bs58::decode(&req.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Ok(error_response("Invalid secret key format")),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return Ok(error_response("Invalid secret key")),
    };

    // Sign the message
    let message_bytes = req.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    Ok(success_response(json!({
        "signature": general_purpose::STANDARD.encode(signature.as_ref()),
        "public_key": bs58::encode(keypair.pubkey().to_bytes()).into_string(),
        "message": req.message
    })))
}

// Endpoint: POST /message/verify
pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> Result<HttpResponse> {
    // Check for missing required fields
    if req.message.is_empty() || req.signature.is_empty() || req.pubkey.is_empty() {
        return Ok(error_response("Missing required fields"));
    }

    // Parse public key
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Ok(error_response("Invalid public key format")),
    };

    let pubkey = match Pubkey::try_from(pubkey_bytes.as_slice()) {
        Ok(pk) => pk,
        Err(_) => return Ok(error_response("Invalid public key")),
    };

    // Parse signature
    let signature_bytes = match general_purpose::STANDARD.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Ok(error_response("Invalid signature format")),
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return Ok(error_response("Invalid signature")),
    };

    // Verify signature
    let message_bytes = req.message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    Ok(success_response(json!({
        "valid": is_valid,
        "message": req.message,
        "pubkey": req.pubkey
    })))
}
