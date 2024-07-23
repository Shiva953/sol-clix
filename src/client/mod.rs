// client-side functions for the commands defined in tui related to sol transfer/token transfer, txns, would be defined here
use solana_sdk::{native_token::sol_to_lamports, program_pack::Pack, pubkey::Pubkey, signature::{Keypair, Signature}, signer::Signer, system_instruction, transaction::Transaction};
use spl_token::state::Mint;
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use clearscreen;
use std::{collections::HashMap, str::FromStr};
use anyhow::{self};
use solana_account_decoder::{UiAccountData, parse_account_data::ParsedAccount};
// use serde_json::{Value, from_str};

fn decode_ui_account_data(data: &UiAccountData) -> Option<&ParsedAccount> {
    match data {
        UiAccountData::Json(parsed_account) => Some(parsed_account),
        _ => None,
    }
}

pub fn transfer_sol_or_spl_token(
    connection: &RpcClient, 
    symbol: String, 
    amount: f64, 
    recipient: Pubkey, 
    keypair: &Keypair
) -> Result<Signature, anyhow::Error> 
    {
    let sender = keypair.pubkey();
    let symbol = symbol.to_uppercase();
    let tx_sig: Signature;
    if symbol == "SOL" {
        let transfer_ix = system_instruction::transfer(
            &sender,
            &recipient,
            sol_to_lamports(amount),
        );
    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&sender),
        &[keypair],
        recent_blockhash,
    );
     
    tx_sig = connection.send_and_confirm_transaction(&transaction).expect("Failed to send transaction.");
    }
    else {
        // get token mint address and token decimals, convert amount in that form
        // get ATA of both sender and recipient address
        // if ATA doesn't exist, create it
        // transfer
        let (mint, decimals) = get_token_mint_and_decimals(connection, &symbol).unwrap();
        let base:u32 = 10;
        let amount = amount * (base.pow(decimals as u32) as f64);

        let sender_ta = spl_associated_token_account::get_associated_token_address_with_program_id(&sender, &mint, &spl_token_2022::id());
        let recipient_ta = spl_associated_token_account::get_associated_token_address_with_program_id(&recipient, &mint, &spl_token_2022::id());

        //checking if token account exists, if not then creating it
        ensure_assoc_acc_exists(connection, &sender_ta, &sender, keypair, &mint).unwrap();
        ensure_assoc_acc_exists(connection, &recipient_ta, &recipient, keypair, &mint).unwrap();
        
        let ix = spl_token_2022::instruction::transfer_checked(&spl_token_2022::id(), &sender_ta, &mint, &recipient_ta, &sender, &[&sender], amount as u64, decimals).unwrap();

        let recent_blockhash = connection.get_latest_blockhash().expect("err - failed to get blockhash");
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&sender),
            &[&*keypair],
            recent_blockhash,
        );
        tx_sig = connection.send_and_confirm_transaction(&tx).expect("Failed to send transaction.");
    }
    Ok(tx_sig)
}

pub fn get_token_mint_and_decimals(connection: &RpcClient, symbol: &str) -> Result<(Pubkey, u8), anyhow::Error>{
    
    let mut map:HashMap<&str, &str> = HashMap::new();
    map.insert("USDC", "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");
    map.insert("MEME50", "65qQAmZREgNGuBK9mgrriXqqdndzG8NKNc5hBry7GT3u");
    map.insert("BONK", "A7VzpmpqvLbFTHdPiF1hAdKaf2YyHtmxsAgRZNYvjQxB");
    // add more tokens

    // map.insert("USDC", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let token_mint = map.get(symbol).ok_or(anyhow::anyhow!(
        "token not found in list"
    )).unwrap();
    let mint = Pubkey::from_str(token_mint).unwrap();
    let account_data = connection.get_account_data(&mint).unwrap();
    let decimals:u8 = match Mint::unpack(&account_data.as_slice()){
        core::result::Result::Ok(x) => {
            x.decimals
        },
        Err(e) => {
            println!("{:?}", e);
            2
        }
    };
    Ok((mint, decimals))
}

pub fn get_all_tokens_of_wallet(connection: &RpcClient, keypair: &Keypair) -> Result<Vec<(String,f64)>, anyhow::Error>{
    clearscreen::clear().expect("Failed to clear the screen.");
    let owner = keypair.pubkey();
    let token_accounts_2022 = connection
        .get_token_accounts_by_owner(&owner, TokenAccountsFilter::ProgramId(spl_token_2022::id()))
        .expect("Tokens Not Found for the wallet");

    let token_accounts = connection
        .get_token_accounts_by_owner(&owner, TokenAccountsFilter::ProgramId(spl_token::id()))
        .expect("Tokens Not Found for the wallet");

    let mut token_accs = token_accounts_2022;
    token_accs.extend(token_accounts);

    let mut t_as = Vec::new();
    for token_account in token_accs.iter() {
        let data: &UiAccountData = &token_account.account.data;
        let p_d = &decode_ui_account_data(&data).unwrap().parsed.as_object().unwrap();
        let (mint, balance) = (p_d.get("info").unwrap().get("mint").unwrap().as_str().unwrap().to_string(), p_d.get("info").unwrap().get("tokenAmount").unwrap().get("uiAmount").unwrap().as_number().unwrap().as_f64().unwrap());
        if balance!=0.0 {
            t_as.push((mint, balance));
        }
    }
    Ok(t_as)
}

pub fn ensure_assoc_acc_exists(
    client: &RpcClient,
    token_acc_addr: &Pubkey,
    token_acc_owner_addr: &Pubkey,
    payer: &Keypair,
    token_mint_addr: &Pubkey,
) -> anyhow::Result<()> {

    if let Err(_) = client.get_account(&token_acc_addr) {
        let create_dest_token_acc_ix =
        spl_associated_token_account::instruction::create_associated_token_account(
                &payer.pubkey(),
                &token_acc_owner_addr,
                &token_mint_addr,
                &spl_token_2022::id()
            );
        let tx = Transaction::new_signed_with_payer(
            &[create_dest_token_acc_ix],
            Some(&payer.pubkey()),
            &[&*payer],
            client.get_latest_blockhash().expect("some"),
        );
        let _r = client.send_transaction(&tx)?;
        println!("assoc account created with signature: {:?}", _r);
    };

    Ok(())
}

pub fn request_airdrop(connection: &RpcClient, addr: &Pubkey, amount: f64) -> Result<solana_sdk::signature::Signature, anyhow::Error>{
    let recent_blockhash = &connection.get_latest_blockhash()?;
    let sig = connection.request_airdrop_with_blockhash(addr, sol_to_lamports(amount), recent_blockhash)?;
    connection.confirm_transaction_with_spinner(
        &sig,
        &recent_blockhash,
        connection.commitment(),
    )?;
    Ok(sig)
}