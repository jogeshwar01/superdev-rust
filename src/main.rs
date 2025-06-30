use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use serde_json::json;

// Health check endpoint
async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Server is healthy"
    })))
}

// Configure routes
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
