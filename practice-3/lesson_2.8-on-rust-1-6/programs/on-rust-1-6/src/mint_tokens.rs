use std::{env, process};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signature, Signer, read_keypair_file},
    pubkey::Pubkey,
};
use spl_token::{instruction::mint_to, ID as TOKEN_PROGRAM_ID};
use dotenv::dotenv;
use std::str::FromStr;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load secret key from .env
    let private_key = env::var("SECRET_KEY").expect("❌ Add SECRET_KEY to .env!");

    // Deserialize the key
    let key_bytes: Vec<u8> = serde_json::from_str(&private_key)?;
    let sender = Keypair::from_bytes(&key_bytes)?;

    // Connect to devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Token mint and recipient ATA
    let token_mint_pubkey = Pubkey::from_str("Gbkcir73TGUj4abXG44QqSUJFsQvvwmwEYuyAyF4U9Bz")?;
    let recipient_ata = Pubkey::from_str("4CquhUDqdfEbHiJWwj3zPtPfhxs7mdjQF9b7NJR1LFDb")?;

    // Amount 10.00 tokens with 2 decimals
    let amount = 10_00;

    // Create mint_to instruction
    let mint_ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &token_mint_pubkey,
        &recipient_ata,
        &sender.pubkey(),
        &[],
        amount,
    )?;

    let blockhash = client.get_latest_blockhash()?;
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[mint_ix],
        Some(&sender.pubkey()),
        &[&sender],
        blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!(
        "✅ Success! Mint Token Transaction: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );

    Ok(())
}
