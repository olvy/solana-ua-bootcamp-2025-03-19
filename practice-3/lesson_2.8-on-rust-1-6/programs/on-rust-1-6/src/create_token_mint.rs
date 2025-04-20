use anyhow::Result;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
};
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;
use spl_token::id as token_program_id;
use std::{env, str::FromStr};

pub fn run() -> Result<()> {
    dotenv().ok();

    let secret_key = env::var("SECRET_KEY")
        .expect("❌ Add SECRET_KEY to your .env file!");
    let keypair_bytes: Vec<u8> = serde_json::from_str(&secret_key)?;
    let payer = Keypair::from_bytes(&keypair_bytes)?;

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    println!("🔑 Our public key is: {}", payer.pubkey());

    // new token mint 
    let mint_account = Keypair::new();
    let mint_len = 82; // bytes
    let rent_exemption = connection.get_minimum_balance_for_rent_exemption(mint_len)?;

    let create_account_ix = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &mint_account.pubkey(),
        rent_exemption,
        mint_len as u64,
        &token_program_id(),
    );

    let initialize_mint_ix = initialize_mint(
        &token_program_id(),
        &mint_account.pubkey(),
        &payer.pubkey(),
        None,
        2, // decimals
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint_account],
        recent_blockhash,
    );

    let sig = connection.send_and_confirm_transaction(&tx)?;
    let explorer_link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint_account.pubkey()
    );

    println!("✅ Token Mint: {}", explorer_link);

    Ok(())
}
