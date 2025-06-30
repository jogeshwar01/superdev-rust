use crate::utils::{log_request, log_response, success_response};
use actix_web::{HttpRequest, HttpResponse, Result};
use serde_json::json;

// Health check endpoint
pub async fn health(req: HttpRequest) -> Result<HttpResponse> {
    log_request(&req, "/health", None);

    let data = json!({
        "status": "ok",
        "message": "Solana Fellowship Server is healthy"
    });

    let response = success_response(data.clone());

    // Log the actual wrapped response format
    let wrapped_response = json!({
        "success": true,
        "data": data
    });
    log_response("/health", 200, &wrapped_response.to_string());

    Ok(response)
}
