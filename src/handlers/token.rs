use crate::models::{AccountInfo, CreateTokenRequest, InstructionData, MintTokenRequest};
use crate::utils::{error_response, log_request, log_response, success_response};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Endpoint: POST /token/create
pub async fn create_token(
    req: HttpRequest,
    body: web::Json<CreateTokenRequest>,
) -> Result<HttpResponse> {
    log_request(&req, "/token/create", Some(&json!(&*body)));

    // Validate required fields
    if body.mint_authority.is_empty() || body.mint.is_empty() {
        let error_msg = "Missing required fields";
        log_response(
            "/token/create",
            400,
            &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
        );
        return Ok(error_response(error_msg));
    }

    let mint_authority = match Pubkey::from_str(&body.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_msg = "Invalid mint authority address";
            log_response(
                "/token/create",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_msg = "Invalid mint address";
            log_response(
                "/token/create",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    // Create initialize mint instruction
    let instruction = match token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        body.decimals,
    ) {
        Ok(inst) => inst,
        Err(_) => {
            let error_msg = "Failed to create initialize mint instruction";
            log_response(
                "/token/create",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|account| AccountInfo {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    let instruction_data = InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    let response = success_response(&instruction_data);
    log_response("/token/create", 200, &json!(instruction_data).to_string());

    Ok(response)
}

// Endpoint: POST /token/mint
pub async fn mint_token(
    req: HttpRequest,
    body: web::Json<MintTokenRequest>,
) -> Result<HttpResponse> {
    log_request(&req, "/token/mint", Some(&json!(&*body)));

    // Validate required fields
    if body.mint.is_empty() || body.destination.is_empty() || body.authority.is_empty() {
        let error_msg = "Missing required fields";
        log_response(
            "/token/mint",
            400,
            &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
        );
        return Ok(error_response(error_msg));
    }

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_msg = "Invalid mint address";
            log_response(
                "/token/mint",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let destination = match Pubkey::from_str(&body.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_msg = "Invalid destination address";
            log_response(
                "/token/mint",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let authority = match Pubkey::from_str(&body.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_msg = "Invalid authority address";
            log_response(
                "/token/mint",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    if body.amount == 0 {
        let error_msg = "Amount must be greater than 0";
        log_response(
            "/token/mint",
            400,
            &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
        );
        return Ok(error_response(error_msg));
    }

    // Get associated token account for destination
    let destination_ata = get_associated_token_address(&destination, &mint);

    // Create mint to instruction
    let instruction = match token_instruction::mint_to(
        &spl_token::id(),
        &mint,
        &destination_ata,
        &authority,
        &[],
        body.amount,
    ) {
        Ok(inst) => inst,
        Err(_) => {
            let error_msg = "Failed to create mint to instruction";
            log_response(
                "/token/mint",
                400,
                &format!(r#"{{"success":false,"error":"{}"}}"#, error_msg),
            );
            return Ok(error_response(error_msg));
        }
    };

    let accounts: Vec<AccountInfo> = instruction
        .accounts
        .iter()
        .map(|account| AccountInfo {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    let instruction_data = InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    let response = success_response(&instruction_data);
    log_response("/token/mint", 200, &json!(instruction_data).to_string());

    Ok(response)
}
