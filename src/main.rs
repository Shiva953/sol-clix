use colored::Colorize;
use serde::{Deserialize, Serialize};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::rpc_client::RpcClient;
pub use solana_cli_wallet_neutron::wallet::{gen_sol_wallet, generate_and_save_mnemonic, import_and_save_private_key};
use solana_cli_wallet_neutron::tui::menu::show_menu;

use std::{panic, panic::PanicInfo};

#[derive(Serialize, Deserialize, Debug)]
struct ParsedAccount {
    program: String,
    parsed: Parsed,
    space: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parsed {
    #[serde(rename = "type")]
    account_type: String,
    info: AccountInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccountInfo {
    is_native: bool,
    mint: String,
    owner: String,
    state: String,
    token_amount: TokenAmount,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenAmount {
    amount: String,
    decimals: u8,
    ui_amount: f64,
    ui_amount_string: String,
}

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap(); 

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };
    println!(
        "{}thread '<unnamed>' panicked at '{}', {}\r",
        termion::screen::ToMainScreen,
        msg,
        location
    );
}

fn main(){
    clearscreen::clear().expect("Failed to clear the screen.");
    println!("{}", "Choose your environment".italic());
    println!("{}", "A. Mainnet-beta".black().yellow());
    println!("{}", "B. Devnet(Default)".black().blue());
    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option = option.trim();
    let url: String;
    match option {
        "A" => {
            url = "https://api.mainnet-beta.solana.com".to_string();
        },
        "B" => {
            url = "https://api.devnet.solana.com".to_string();
        },
        _ => {
            url = "https://api.devnet.solana.com".to_string();
        }
    }
    let connection: RpcClient = RpcClient::new_with_commitment(
        url.to_owned(),
        CommitmentConfig::confirmed()
    );
    panic::set_hook(Box::new(panic_hook));
    show_menu(&connection);
}