use crate::utils::{log_request, log_response, success_response};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde_json::json;

// Health check endpoint
pub async fn health(req: HttpRequest) -> Result<HttpResponse> {
    log_request(&req, "/health", None);

    let response_data = json!({
        "status": "ok",
        "message": "Solana Fellowship Server is healthy"
    });

    let response = success_response(response_data.clone());
    log_response("/health", 200, &response_data.to_string());

    Ok(response)
}
