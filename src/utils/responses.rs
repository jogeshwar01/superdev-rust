use actix_web::HttpResponse;
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;

// Helper function to create success response with guaranteed field order
pub fn success_response<T: Serialize>(data: T) -> HttpResponse {
    let mut response = IndexMap::new();
    response.insert("success", Value::Bool(true));
    response.insert("data", serde_json::to_value(data).unwrap());

    HttpResponse::Ok().json(response)
}

// Helper function to create error response with guaranteed field order
pub fn error_response(error: &str) -> HttpResponse {
    let mut response = IndexMap::new();
    response.insert("success", Value::Bool(false));
    response.insert("error", Value::String(error.to_string()));

    HttpResponse::BadRequest().json(response)
}
