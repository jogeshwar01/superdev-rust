use actix_web::{middleware::Logger, App, HttpServer};

mod handlers;
mod models;
mod routes;
mod utils;

use routes::configure_routes;
use utils::log_startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Log server startup
    log_startup();

    println!("🚀 Starting Solana Fellowship Server at http://0.0.0.0:8080");
    println!("\nAvailable Endpoints:");
    println!("  GET  /health                - Health check");
    println!("  POST /keypair               - Generate new keypair");
    println!("  POST /token/create          - Create SPL token mint instruction");
    println!("  POST /token/mint            - Create SPL token mint-to instruction");
    println!("  POST /message/sign          - Sign message with private key");
    println!("  POST /message/verify        - Verify signed message");
    println!("  POST /send/sol              - Create SOL transfer instruction");
    println!("  POST /send/token            - Create SPL token transfer instruction");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
