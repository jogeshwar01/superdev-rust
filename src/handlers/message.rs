use crate::models::{SignMessageRequest, VerifyMessageRequest};
use crate::utils::{error_response, log_request, log_response, success_response};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Signature, Signer},
    signer::keypair::Keypair,
};

// Endpoint: POST /message/sign
pub async fn sign_message(
    req: HttpRequest,
    body: web::Json<SignMessageRequest>,
) -> Result<HttpResponse> {
    log_request(&req, "/message/sign", Some(&json!(&*body)));

    // Check for missing required fields
    if body.message.is_empty() || body.secret.is_empty() {
        let error_msg = "Missing required fields";
        log_response(
            "/message/sign",
            400,
            &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
        );
        return Ok(error_response(error_msg));
    }

    // Decode the secret key
    let secret_bytes = match bs58::decode(&body.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            let error_msg = "Invalid secret key format";
            log_response(
                "/message/sign",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            let error_msg = "Invalid secret key";
            log_response(
                "/message/sign",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    // Sign the message
    let message_bytes = body.message.as_bytes();
    let signature = keypair.sign_message(message_bytes);

    let response_data = json!({
        "signature": general_purpose::STANDARD.encode(signature.as_ref()),
        "public_key": bs58::encode(keypair.pubkey().to_bytes()).into_string(),
        "message": body.message
    });

    let response = success_response(response_data.clone());
    log_response("/message/sign", 200, &response_data.to_string());

    Ok(response)
}

// Endpoint: POST /message/verify
pub async fn verify_message(
    req: HttpRequest,
    body: web::Json<VerifyMessageRequest>,
) -> Result<HttpResponse> {
    log_request(&req, "/message/verify", Some(&json!(&*body)));

    // Check for missing required fields
    if body.message.is_empty() || body.signature.is_empty() || body.pubkey.is_empty() {
        let error_msg = "Missing required fields";
        log_response(
            "/message/verify",
            400,
            &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
        );
        return Ok(error_response(error_msg));
    }

    // Parse public key
    let pubkey_bytes = match bs58::decode(&body.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            let error_msg = "Invalid public key format";
            log_response(
                "/message/verify",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let pubkey = match Pubkey::try_from(pubkey_bytes.as_slice()) {
        Ok(pk) => pk,
        Err(_) => {
            let error_msg = "Invalid public key";
            log_response(
                "/message/verify",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    // Parse signature
    let signature_bytes = match general_purpose::STANDARD.decode(&body.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            let error_msg = "Invalid signature format";
            log_response(
                "/message/verify",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            let error_msg = "Invalid signature";
            log_response(
                "/message/verify",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    // Verify signature
    let message_bytes = body.message.as_bytes();
    let is_valid = signature.verify(&pubkey.to_bytes(), message_bytes);

    let response_data = json!({
        "valid": is_valid,
        "message": body.message,
        "pubkey": body.pubkey
    });

    let response = success_response(response_data.clone());
    log_response("/message/verify", 200, &response_data.to_string());

    Ok(response)
}
