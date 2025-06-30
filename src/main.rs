use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use serde_json::json;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::str::FromStr;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Server is healthy"
    })))
}

async fn balance(path: web::Path<String>) -> Result<HttpResponse> {
    let wallet_address = path.into_inner();

    let pubkey = match Pubkey::from_str(&wallet_address) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(json!({
                "error": "Invalid wallet address format"
            })));
        }
    };

    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

    let rpc_client = RpcClient::new(rpc_url);

    match rpc_client.get_balance(&pubkey).await {
        Ok(lamports) => {
            let sol_balance = lamports as f64 / 1_000_000_000.0;
            Ok(HttpResponse::Ok().json(json!({
                "wallet_address": wallet_address,
                "balance_lamports": lamports,
                "balance_sol": sol_balance
            })))
        }
        Err(err) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch balance: {}", err)
        }))),
    }
}

fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/balance/{wallet_address}", web::get().to(balance));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let rpc_url = env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());

    println!("Starting server at http://0.0.0.0:8080");
    println!("Using Solana RPC: {}", rpc_url);
    println!("Endpoints:");
    println!("  GET /health - Health check");
    println!("  GET /balance/{{wallet_address}} - Get Solana wallet balance");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
