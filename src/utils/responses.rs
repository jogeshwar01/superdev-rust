use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

// Helper function to create success response
pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "data": data
    }))
}

// Helper function to create error response
pub fn error_response(error: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "success": false,
        "error": error
    }))
}
