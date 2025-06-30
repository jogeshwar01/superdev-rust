use crate::models::{AccountInfo, InstructionData, SendSolRequest, SendTokenRequest};
use crate::utils::{error_response, success_response};
use actix_web::{web, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Endpoint: POST /send/sol
pub async fn send_sol(req: web::Json<SendSolRequest>) -> Result<HttpResponse> {
    let from = match Pubkey::from_str(&req.from) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid from address")),
    };

    let to = match Pubkey::from_str(&req.to) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid to address")),
    };

    if req.lamports == 0 {
        return Ok(error_response("Amount must be greater than 0"));
    }

    // Create transfer instruction
    let instruction = system_instruction::transfer(&from, &to, req.lamports);

    let accounts = vec![
        AccountInfo {
            pubkey: instruction.accounts[0].pubkey.to_string(),
            is_signer: instruction.accounts[0].is_signer,
            is_writable: instruction.accounts[0].is_writable,
        },
        AccountInfo {
            pubkey: instruction.accounts[1].pubkey.to_string(),
            is_signer: instruction.accounts[1].is_signer,
            is_writable: instruction.accounts[1].is_writable,
        },
    ];

    let instruction_data = InstructionData {
        program_id: instruction.program_id.to_string(),
        accounts,
        instruction_data: general_purpose::STANDARD.encode(&instruction.data),
    };

    Ok(success_response(instruction_data))
}

// Endpoint: POST /send/token
pub async fn send_token(req: web::Json<SendTokenRequest>) -> Result<HttpResponse> {
    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid destination address")),
    };

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid mint address")),
    };

    let owner = match Pubkey::from_str(&req.owner) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid owner address")),
    };

    if req.amount == 0 {
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
        req.amount,
    ) {
        Ok(inst) => inst,
        Err(_) => return Ok(error_response("Failed to create transfer instruction")),
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

    Ok(success_response(instruction_data))
}
