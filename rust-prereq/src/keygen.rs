use solana_sdk::signature::{Keypair, Signer};

pub fn _keygen() -> Keypair {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey());
    println!();
    println!("To save your wallet, copy and paste the following into a JSON file:");

    println!("{:?}", kp.to_bytes());
    kp
}
