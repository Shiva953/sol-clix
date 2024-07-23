use crossterm::style::Stylize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use crate::client::get_all_tokens_of_wallet;
use crate::tui::menu::main_menu;


pub fn get_all_tokens(connection: &RpcClient, keypair: &Keypair){
    clearscreen::clear().expect("Failed to clear the screen.");
    // let tokens = get_all_tokens_of_wallet(connection, keypair);
    

    match get_all_tokens_of_wallet(connection, keypair){
        Ok(tokens) => {
            print!("{}", "Token Mint".green());
            print!("                                               ");
            print!("{}", "Token Balance".blue());
            println!("\n");
            for token in tokens.iter(){
                print!("{}", token.0.clone().cyan());
                print!("               ");
                print!("{}", token.1.to_string().bold().magenta());
                println!("\n");
            }
        },
        Err(e) => {
            panic!("{:?}", e);
        }
    }
    println!("{}", "Press any key to open the newly generated wallet...");
    let mut any_key = String::new();
    std::io::stdin().read_line(&mut any_key).unwrap();
    main_menu(connection, keypair);
    
}