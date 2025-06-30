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
        let error_response_json = json!({
            "success": false,
            "error": "Missing required fields"
        });
        log_response("/token/create", 400, &error_response_json.to_string());
        return Ok(error_response("Missing required fields"));
    }

    let mint_authority = match Pubkey::from_str(&body.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid mint authority address"
            });
            log_response("/token/create", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid mint authority address"));
        }
    };

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid mint address"
            });
            log_response("/token/create", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid mint address"));
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
            let error_response_json = json!({
                "success": false,
                "error": "Failed to create initialize mint instruction"
            });
            log_response("/token/create", 400, &error_response_json.to_string());
            return Ok(error_response(
                "Failed to create initialize mint instruction",
            ));
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

    // Log the actual wrapped response format
    let wrapped_response = json!({
        "success": true,
        "data": instruction_data
    });
    log_response("/token/create", 200, &wrapped_response.to_string());

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
        let error_response_json = json!({
            "success": false,
            "error": "Missing required fields"
        });
        log_response("/token/mint", 400, &error_response_json.to_string());
        return Ok(error_response("Missing required fields"));
    }

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid mint address"
            });
            log_response("/token/mint", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid mint address"));
        }
    };

    let destination = match Pubkey::from_str(&body.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid destination address"
            });
            log_response("/token/mint", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid destination address"));
        }
    };

    let authority = match Pubkey::from_str(&body.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid authority address"
            });
            log_response("/token/mint", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid authority address"));
        }
    };

    if body.amount == 0 {
        let error_response_json = json!({
            "success": false,
            "error": "Amount must be greater than 0"
        });
        log_response("/token/mint", 400, &error_response_json.to_string());
        return Ok(error_response("Amount must be greater than 0"));
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
            let error_response_json = json!({
                "success": false,
                "error": "Failed to create mint to instruction"
            });
            log_response("/token/mint", 400, &error_response_json.to_string());
            return Ok(error_response("Failed to create mint to instruction"));
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

    // Log the actual wrapped response format
    let wrapped_response = json!({
        "success": true,
        "data": instruction_data
    });
    log_response("/token/mint", 200, &wrapped_response.to_string());

    Ok(response)
}
