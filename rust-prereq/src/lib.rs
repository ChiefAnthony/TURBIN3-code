use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Signer, signer::keypair::read_keypair_file, system_program};
use std::io::{self, BufRead};
mod keygen;
mod programs;
mod register;
use crate::programs::new_prereq::{CompleteArgs, TurbinePrereqProgram};
// use crate::programs::turbin3_prereq::{CompleteArgs, WbaPrereqProgram};

const RPC_URL: &str = "https://api.devnet.solana.com";

pub async fn airdrop() {
    let keypair = read_keypair_file("./dev-wallet.json").expect("Could not find wallet file");
    let client = RpcClient::new(RPC_URL.to_string());
    match client
        .request_airdrop(&keypair.pubkey(), 2_000_000_000u64)
        .await
    {
        Ok(s) => {
            println!("Airdrop success! TX: {}", s);

            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s);
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn base58_to_wallet() {
    println!("Input your private key as base58");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("You wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

pub fn wallet_to_base58() {
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}

pub async fn submit_completion() {
    let signer = read_keypair_file("./wallet.json").expect("could not find turbin3 wallet file");

    let rpc_client = RpcClient::new(RPC_URL.to_string());

    let github_account = "ChiefAnthony";
    let github_bytes = github_account.as_bytes().to_vec();

    let prereq = TurbinePrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref(),
    ]);

    let args = CompleteArgs {
        github: github_bytes,
    };

    let blockhash = match rpc_client.get_latest_blockhash().await {
        Ok(blockhash) => {
            println!("Blockhash retrieved successfully: {}", blockhash);
            blockhash
        }
        Err(e) => {
            eprintln!("Failed to get recent blockhash: {}", e);
            return; // Exit the function if blockhash retrieval fails
        }
    };

    // create complete instruction
    let transaction = TurbinePrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );

    // Send transaction
    let signature = match rpc_client.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction sent successfully. Signature: {}", signature);
            signature
        }
        Err(e) => {
            eprintln!("Failed to send transaction: {}", e);
            return; // Exit the function on failure
        }
    };

    println!(
        "Transaction confirmed: https://explorer.solana.com/tx/{}?cluster=devnet",
        signature
    );
}

#[cfg(test)]
mod tests {
    use super::keygen::_keygen;
    use super::*;
    use solana_sdk::{self, signer::Signer};

    #[test]
    fn keygen() {
        let keypair = _keygen();
        assert!(keypair.pubkey().to_string().is_empty());
    }
    #[tokio::test]
    async fn test_airdrop() {
        airdrop().await;
    }
    #[test]
    fn transfer_sol() {}

    #[test]
    fn test_wallet_to_base58() {
        wallet_to_base58();
    }

    #[test]
    fn test_base58_to_wallet() {
        base58_to_wallet();
    }
}
