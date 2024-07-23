use clearscreen;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
pub use crate::wallet::{gen_sol_wallet, generate_and_save_mnemonic, import_and_save_private_key};
use solana_account_decoder::UiAccountData;
use crate::tui::{import::import_keypair, create::create_keypair, airdrop::airdrop, sol_txn::transfer_token};
use colored::Colorize;
pub fn show_menu(connection: &RpcClient){
    clearscreen::clear().expect("Failed to clear the screen.");
    println!("-------------------------------- WELCOME --------------------------------");
    println!("{}", "1. Create A New Keypair".bold().bright_cyan());
    println!("{}", "2. Import Existing Keypair".bold().bright_green());
    println!("\n");
    println!("Press Any Other Key to Exit...");
    
    loop {
        let mut option = String::new();
        std::io::stdin().read_line(&mut option).expect("Failed to read line");
        let number: u8 = option.trim().parse().unwrap_or_default();

        match number{
            1 => {
                let wallet_keypair = create_keypair().unwrap();
                println!("Press any key to open the newly generated wallet...");
                let mut any_key = String::new();
                std::io::stdin().read_line(&mut any_key).unwrap();
                main_menu(connection, &wallet_keypair);
            },
            2 => {
                // import keypair
                let wallet_keypair = import_keypair();
                println!("Press any key to open the newly generated wallet...");
                let mut any_key = String::new();
                std::io::stdin().read_line(&mut any_key).unwrap();
                main_menu(connection, &wallet_keypair);
            },
            _ => break
        }
    }
}

pub fn main_menu(connection: &RpcClient, keypair: &Keypair){
    clearscreen::clear().expect("Failed to clear the screen.");
    loop {
    //show balance, txn history
    clearscreen::clear().expect("Failed to clear the screen.");
    println!("{}", "----------------------------------------------------------".purple().bold());
    println!("{}", "                        WALLET 1                           ".purple().bold());
    println!("{}", "----------------------------------------------------------".purple().bold());
    println!("Account: {}", keypair.pubkey().to_string().bold().blue());

    let balance = connection.get_balance(&keypair.pubkey()).unwrap();
    let sol_balance = balance as f64/ (1e9);
    println!("{} SOL", sol_balance.to_string().bold().blue());

    println!("\n\n");
    println!("{}", "1. View Your Holdings".bright_magenta().italic());
    println!("{}", "2. Transfer SOL/SPL-Tokens".bright_red().italic());
    println!("{}", "3. Request Airdrop(Only on Devnet)".italic().on_bright_blue());
    println!("\n");
    println!("{}", "Press Any Other Key to Exit".bright_yellow());

    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let number: u8 = option.trim().parse().unwrap_or_default();

    match number {
        1 => {
            // get_all_tokens_of_wallet(connection, &keypair);
        },
        2 => {
            transfer_token(connection, &keypair);
        },
        3 => {
            airdrop(connection, &keypair);
        }
        _ => break
    }
}
}