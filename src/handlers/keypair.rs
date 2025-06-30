use crate::utils::{log_request, log_response, success_response};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde_json::json;
use solana_sdk::signer::{keypair::Keypair, Signer};

// Endpoint: POST /keypair
pub async fn generate_keypair(req: HttpRequest) -> Result<HttpResponse> {
    log_request(&req, "/keypair", None);

    let keypair = Keypair::new();
    let pubkey = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let response_data = json!({
        "pubkey": pubkey,
        "secret": secret
    });

    let response = success_response(response_data.clone());
    log_response("/keypair", 200, &response_data.to_string());

    Ok(response)
}
