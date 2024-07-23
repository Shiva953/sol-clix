use clearscreen;
use colored::Colorize;
use solana_client::rpc_client::RpcClient;
use std::{io, str::FromStr};
use spinoff::{Spinner, spinners, Color};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair};
use crate::client::transfer_sol_or_spl_token;
use crate::tui::menu::main_menu;

pub fn transfer_token(connection: &RpcClient, keypair: &Keypair){
    clearscreen::clear().expect("Failed to clear the screen.");
    println!("{}", "\n===================SPL-Token Transfers ===================\n".bold());
    println!("{}", "Which token do you wanna transfer?".italic().cyan());
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol).unwrap();
    println!("{}", "Recipient Address".italic().cyan());
    let mut rp = String::new();
    
    io::stdin().read_line(&mut rp).unwrap();
    let recipient = Pubkey::from_str(rp.trim()).unwrap();

    println!("{}", "Amount".italic().cyan());
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();
    let final_amount = amount.trim().parse::<f64>().expect("Please enter a valid token amount.");

    let mut sp = Spinner::new(
        spinners::Dots,
        "Creating txn, please wait...",
        Color::White
    );
    let tx_sig = transfer_sol_or_spl_token(connection, symbol.trim().to_string(), final_amount, recipient, keypair).unwrap();
    sp.update(spinners::Dots, "Confirming Transaction...", Color::Cyan);
    loop {
        let result = connection.confirm_transaction_with_commitment(
            &tx_sig,
            CommitmentConfig::finalized()
        ).unwrap();
        if result.value {
            break;
        };
    };
    sp.stop_and_persist("🚀", "Transaction sent successfully.");
    let sign = tx_sig.to_string();
    println!("Transaction signature : {}", sign.bold().bright_magenta());

    println!("{}", "Press any key to open the newly generated wallet...".italic());
    let mut any_key = String::new();
    std::io::stdin().read_line(&mut any_key).unwrap();
    main_menu(connection, keypair);
}