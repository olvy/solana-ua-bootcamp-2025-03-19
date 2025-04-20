use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    instruction::{Instruction, AccountMeta},
    transaction::Transaction,
    pubkey::Pubkey,
};
use anyhow::Result;
use std::env;
use dotenv::dotenv;

pub fn run() -> Result<()> {
    dotenv().ok();

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let secret_key_json = env::var("SECRET_KEY")?;
    let secret_key_vec: Vec<u8> = serde_json::from_str(&secret_key_json)?;
    let sender = Keypair::from_bytes(&secret_key_vec)?;
    let sender_pubkey = sender.pubkey();

    println!("🔑 Our public key is: {}", sender_pubkey);

    let recipient_pubkey: Pubkey = "ToJMdQ5onKqSUDeNf2zJHuqpBkp8FcnKm8So1B6dHei".parse()?;
    let lamports = 100_000; // 0.0001 SOL

    let transfer_ix = system_instruction::transfer(&sender_pubkey, &recipient_pubkey, lamports);
    let recent_blockhash = client.get_latest_blockhash()?;

    let hello_world_ix = Instruction {
        program_id: "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr".parse()?, 
        accounts: vec![
            AccountMeta::new(sender_pubkey, true),
        ],
        data: b"Hello Solana world!".to_vec(),
    };

    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix, hello_world_ix],
        Some(&sender_pubkey),
        &[&sender],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("✅ Transaction confirmed, signature: {}", signature);

    Ok(())
}
