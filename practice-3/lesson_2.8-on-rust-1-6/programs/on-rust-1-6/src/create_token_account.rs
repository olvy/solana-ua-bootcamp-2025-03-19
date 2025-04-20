use anyhow::Result;
use dotenv::dotenv;
use std::env;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account_idempotent,
};

use spl_token::id as TOKEN_PROGRAM_ID;
use spl_associated_token_account::id as ASSOCIATED_TOKEN_PROGRAM_ID;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program,
};

fn build_create_ata_ix(
    payer: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Instruction {
    let ata = spl_associated_token_account::get_associated_token_address(owner, mint);

    let accounts = vec![
        AccountMeta::new(*payer, true),                      // payer
        AccountMeta::new(ata, false),                        // associated token account
        AccountMeta::new_readonly(*owner, false),           // token owner
        AccountMeta::new_readonly(*mint, false),            // mint
        AccountMeta::new_readonly(system_program::id(), false), // system program
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID(), false),   // token program
        AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false), // rent sysvar
    ];

    Instruction {
        program_id: ASSOCIATED_TOKEN_PROGRAM_ID(),
        accounts,
        data: vec![], // associated token account program uses empty data
    }
}

pub fn run() -> Result<()> {
    dotenv().ok();

    // Load and parse SECRET_KEY from .env
    let secret_key = env::var("SECRET_KEY").expect("❌ Add SECRET_KEY to .env!");
    let keypair_bytes: Vec<u8> = serde_json::from_str(&secret_key)?;
    let sender = Keypair::from_bytes(&keypair_bytes)?;

    // Setup RPC connection to Devnet
    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    println!("🔑 Our public key is: {}", sender.pubkey());

    // Token Mint and Recipient
    let token_mint_account: Pubkey = "Gbkcir73TGUj4abXG44QqSUJFsQvvwmwEYuyAyF4U9Bz".parse()?;
    let recipient: Pubkey = "EGEvS5e1idda5zpUs29asB1EB9qfvqzKrNsT8UCS1DDa".parse()?;
    // let spl_ata_program_id: Pubkey = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL".parse()?;

    // Derive the associated token account
    let ata_address = get_associated_token_address(&recipient, &token_mint_account);

    // Check if it exists
    if connection.get_account(&ata_address).is_err() {
        println!("📦 Creating associated token account: {}", ata_address);

        let create_ix = build_create_ata_ix(
            &sender.pubkey(),
            &recipient,
            &token_mint_account,
        );

        let blockhash = connection.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&sender.pubkey()),
            &[&sender],
            blockhash,
        );

        connection.send_and_confirm_transaction(&tx)?;
        println!("✅ Created token account!");
    } else {
        println!("ℹ️ Associated token account already exists.");
    }

    println!(
        "🔗 Explorer: https://explorer.solana.com/address/{}?cluster=devnet",
        ata_address
    );

    Ok(())
}
