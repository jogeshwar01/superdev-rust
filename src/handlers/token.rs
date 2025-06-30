use crate::models::{AccountInfo, CreateTokenRequest, InstructionData, MintTokenRequest};
use crate::utils::{error_response, success_response};
use actix_web::{web, HttpResponse, Result};
use base64::{engine::general_purpose, Engine};
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction as token_instruction;
use std::str::FromStr;

// Endpoint: POST /token/create
pub async fn create_token(req: web::Json<CreateTokenRequest>) -> Result<HttpResponse> {
    // Validate required fields
    if req.mint_authority.is_empty() || req.mint.is_empty() {
        return Ok(error_response("Missing required fields"));
    }

    let mint_authority = match Pubkey::from_str(&req.mint_authority) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid mint authority address")),
    };

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid mint address")),
    };

    // Create initialize mint instruction
    let instruction = match token_instruction::initialize_mint(
        &spl_token::id(),
        &mint,
        &mint_authority,
        Some(&mint_authority),
        req.decimals,
    ) {
        Ok(inst) => inst,
        Err(_) => {
            return Ok(error_response(
                "Failed to create initialize mint instruction",
            ))
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

    Ok(success_response(instruction_data))
}

// Endpoint: POST /token/mint
pub async fn mint_token(req: web::Json<MintTokenRequest>) -> Result<HttpResponse> {
    // Validate required fields
    if req.mint.is_empty() || req.destination.is_empty() || req.authority.is_empty() {
        return Ok(error_response("Missing required fields"));
    }

    let mint = match Pubkey::from_str(&req.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid mint address")),
    };

    let destination = match Pubkey::from_str(&req.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid destination address")),
    };

    let authority = match Pubkey::from_str(&req.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => return Ok(error_response("Invalid authority address")),
    };

    if req.amount == 0 {
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
        req.amount,
    ) {
        Ok(inst) => inst,
        Err(_) => return Ok(error_response("Failed to create mint to instruction")),
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
