use crate::models::{
    AccountInfoCamelCase, SendSolRequest, SendTokenRequest, SolTransferData, TokenTransferData,
};
use crate::utils::{error_response, log_request, log_response, success_response};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use serde_json::json;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Endpoint: POST /send/sol
pub async fn send_sol(req: HttpRequest, body: web::Json<SendSolRequest>) -> Result<HttpResponse> {
    log_request(&req, "/send/sol", Some(&json!(&*body)));

    // Validate required fields
    if body.from.is_empty() || body.to.is_empty() {
        let error_response_json = json!({
            "success": false,
            "error": "Missing required fields"
        });
        log_response("/send/sol", 400, &error_response_json.to_string());
        return Ok(error_response("Missing required fields"));
    }

    let from = match Pubkey::from_str(&body.from) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid from address"
            });
            log_response("/send/sol", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid from address"));
        }
    };

    let to = match Pubkey::from_str(&body.to) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid to address"
            });
            log_response("/send/sol", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid to address"));
        }
    };

    if body.lamports == 0 {
        let error_response_json = json!({
            "success": false,
            "error": "Amount must be greater than 0"
        });
        log_response("/send/sol", 400, &error_response_json.to_string());
        return Ok(error_response("Amount must be greater than 0"));
    }

    // Create transfer instruction
    let instruction = system_instruction::transfer(&from, &to, body.lamports);

    let accounts = vec![
        instruction.accounts[0].pubkey.to_string(),
        instruction.accounts[1].pubkey.to_string(),
    ];

    let instruction_data = SolTransferData {
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
    log_response("/send/sol", 200, &wrapped_response.to_string());

    Ok(response)
}

// Endpoint: POST /send/token
pub async fn send_token(
    req: HttpRequest,
    body: web::Json<SendTokenRequest>,
) -> Result<HttpResponse> {
    log_request(&req, "/send/token", Some(&json!(&*body)));

    // Validate required fields
    if body.destination.is_empty() || body.mint.is_empty() || body.owner.is_empty() {
        let error_response_json = json!({
            "success": false,
            "error": "Missing required fields"
        });
        log_response("/send/token", 400, &error_response_json.to_string());
        return Ok(error_response("Missing required fields"));
    }

    let destination = match Pubkey::from_str(&body.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid destination address"
            });
            log_response("/send/token", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid destination address"));
        }
    };

    let mint = match Pubkey::from_str(&body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid mint address"
            });
            log_response("/send/token", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid mint address"));
        }
    };

    let owner = match Pubkey::from_str(&body.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Invalid owner address"
            });
            log_response("/send/token", 400, &error_response_json.to_string());
            return Ok(error_response("Invalid owner address"));
        }
    };

    if body.amount == 0 {
        let error_response_json = json!({
            "success": false,
            "error": "Amount must be greater than 0"
        });
        log_response("/send/token", 400, &error_response_json.to_string());
        return Ok(error_response("Amount must be greater than 0"));
    }

    // Get associated token accounts
    let source_ata = get_associated_token_address(&owner, &mint);
    let destination_ata = get_associated_token_address(&destination, &mint);

    // Create transfer instruction
    let instruction = match token_instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &destination_ata,
        &owner,
        &[],
        body.amount,
    ) {
        Ok(inst) => inst,
        Err(_) => {
            let error_response_json = json!({
                "success": false,
                "error": "Failed to create transfer instruction"
            });
            log_response("/send/token", 400, &error_response_json.to_string());
            return Ok(error_response("Failed to create transfer instruction"));
        }
    };

    let accounts: Vec<AccountInfoCamelCase> = instruction
        .accounts
        .iter()
        .map(|account| AccountInfoCamelCase {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
        })
        .collect();

    let instruction_data = TokenTransferData {
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
    log_response("/send/token", 200, &wrapped_response.to_string());

    Ok(response)
}
