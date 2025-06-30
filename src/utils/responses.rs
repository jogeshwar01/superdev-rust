use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "success": true,
        "data": data
    }))
}

pub fn error_response(error: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "success": false,
        "error": error
    }))
}
