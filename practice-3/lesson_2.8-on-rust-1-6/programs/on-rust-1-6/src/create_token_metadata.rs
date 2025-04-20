use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    sysvar,
    transaction::Transaction,
};
use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs},
    types::DataV2,
};
use std::{env, str::FromStr};
use serde_json;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // 🔑 Load keypair from environment variable
    let private_key = env::var("SECRET_KEY").expect("❌ Add SECRET_KEY to .env!");
    let key_bytes: Vec<u8> = serde_json::from_str(&private_key)?;
    let payer = Keypair::from_bytes(&key_bytes)?;

    // 🔌 Connect to Solana Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    // 🪙 Token mint address
    let mint_pubkey = Pubkey::from_str("Gbkcir73TGUj4abXG44QqSUJFsQvvwmwEYuyAyF4U9Bz")?;

    // 🎨 Token Metadata Program ID
    let metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")?;

    // 📦 Derive PDA for metadata account
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_program_id.as_ref(),
            mint_pubkey.as_ref(),
        ],
        &metadata_program_id,
    );

    // 📄 Metadata fields
    let metadata_data = DataV2 {
        name: "Solana UA Bootcamp 2025-03-19".to_string(),
        symbol: "UAB-3".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // 🧱 Create the instruction with correct builder
    let instruction = CreateMetadataAccountV3 {
        metadata: metadata_pda,
        mint: mint_pubkey,
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: system_program::id(),
        rent: Some(sysvar::rent::id()),
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: None,
    });

    // 📤 Send transaction
    let blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;
    println!(
        "✅ Success! View on explorer:\nhttps://explorer.solana.com/tx/{}?cluster=devnet",
        sig
    );

    Ok(())
}
