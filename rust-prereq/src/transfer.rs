use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};

use solana_sdk::{
    feature_set::add_get_minimum_delegation_instruction_to_stake_program, message::Message,
    signature::Signer, signer::keypair::read_keypair_file, system_instruction,
    transaction::Transaction,
};

use std::str::FromStr;

pub fn _transfer_sol() {
    let keypair = read_keypair_file("./dev-wallet.json").expect("Could not find wallet file");
    let to_pubkey = Pubkey::from_str("9L59nmqeU4ju3iJ2ybab1JTUkQRQiJ41RoxFhSccW1i7").unwrap();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

pub fn _transfer_all_to_turbin3() {
    let keypair = read_keypair_file("./dev-wallet.json").expect("Could not find wallet file");
    let to_pubkey = Pubkey::from_str("9L59nmqeU4ju3iJ2ybab1JTUkQRQiJ41RoxFhSccW1i7").unwrap();
    let rpc_client = RpcClient::new("https://api.devnet.solana.com");
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("failed to get recent blockhash");

    // Get balance of dev wallet

    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    // create transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    // calculate exact fee amount
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
