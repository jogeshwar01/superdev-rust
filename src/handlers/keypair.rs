use crate::utils::success_response;
use actix_web::{HttpResponse, Result};
use serde_json::json;
use solana_sdk::signer::{keypair::Keypair, Signer};

pub async fn generate_keypair() -> Result<HttpResponse> {
    let keypair = Keypair::new();
    let pubkey = bs58::encode(keypair.pubkey().to_bytes()).into_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    Ok(success_response(json!({
        "pubkey": pubkey,
        "secret": secret
    })))
}
