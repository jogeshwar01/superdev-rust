use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateTokenRequest {
    #[serde(alias = "mintAuthority", alias = "mint_authority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Deserialize, Serialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Deserialize, Serialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Deserialize, Serialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}
