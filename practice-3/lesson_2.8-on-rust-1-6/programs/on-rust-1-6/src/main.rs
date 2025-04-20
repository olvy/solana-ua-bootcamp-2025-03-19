mod send_sol;
mod create_token_mint;
mod create_token_account;
mod mint_tokens;
mod create_token_metadata;

fn main() {
    // if let Err(err) = send_sol::run() {
    //     eprintln!("❌ Error: {:?}", err);
    // }
    // if let Err(err) = create_token_mint::run() {
    //     eprintln!("❌ Error: {:?}", err);
    // }
    // if let Err(err) = create_token_account::run() {
    //     eprintln!("❌ Error: {:?}", err);
    // }
    // if let Err(err) = mint_tokens::run() {
    //     eprintln!("❌ Error: {:?}", err);
    // }
    if let Err(err) = create_token_metadata::run() {
        eprintln!("❌ Error: {:?}", err);
    }
}