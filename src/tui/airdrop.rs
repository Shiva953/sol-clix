use clearscreen;
use crossterm::style::Stylize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer};
use spinoff::{Spinner, spinners, Color};
use crate::client::request_airdrop;
use crate::tui::menu::main_menu;

pub fn airdrop(connection: &RpcClient, keypair: &Keypair){
    clearscreen::clear().expect("Failed to clear the screen.");
    println!("\n===================Request Airdrop ===================\n");
            // println!("Enter address to airdrop to : ");
            // let mut rp = String::new();
            // io::stdin().read_line(&mut rp).unwrap();
            // let addr = Pubkey::from_str(rp.trim()).unwrap();
            println!("{}", "Amount(in SOL)".dark_blue().bold());
            let mut amount = String::new();
            std::io::stdin().read_line(&mut amount).unwrap();
            let final_amount = amount.trim().parse::<f64>().expect("Please enter a valid sol amount.");
            
            let mut sp = Spinner::new(
                spinners::Dots,
                "Creating airdrop, please wait...",
                Color::Green
            );
            sp.update(spinners::Dots, "Confirming Airdrop", Color::Magenta);

            let sign = request_airdrop(connection, &keypair.pubkey(), final_amount).unwrap();
            sp.stop_and_persist("🚀", "Airdrop Successful.");
            println!("{:?}", sign.to_string());

            println!("{}", "Press Any Key to Return to Menu.");
            let mut any_key = String::new();
            std::io::stdin().read_line(&mut any_key).unwrap();
            main_menu(connection, keypair);
}