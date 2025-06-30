use crate::utils::success_response;
use actix_web::{HttpResponse, Result};
use serde_json::json;

// Health check endpoint
pub async fn health() -> Result<HttpResponse> {
    Ok(success_response(json!({
        "status": "ok",
        "message": "Solana Fellowship Server is healthy"
    })))
}
