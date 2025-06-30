use crate::handlers::{
    health::health,
    keypair::generate_keypair,
    message::{sign_message, verify_message},
    token::{create_token, mint_token},
    transfer::{send_sol, send_token},
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/keypair", web::post().to(generate_keypair))
        .route("/token/create", web::post().to(create_token))
        .route("/token/mint", web::post().to(mint_token))
        .route("/message/sign", web::post().to(sign_message))
        .route("/message/verify", web::post().to(verify_message))
        .route("/send/sol", web::post().to(send_sol))
        .route("/send/token", web::post().to(send_token));
}
