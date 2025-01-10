// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     signature::{read_keypair_file, Signer},
//     system_program,
// };
//
// use crate::programs::turbin3_prereq::{CompleteArgs, WbaPrereqProgram};
//
// pub fn submit_completion() {
//     let signer = read_keypair_file("./turbin3-wallet.json").expect("Could not find wallet file");
//
//     let rpc_client = RpcClient::new("https:/api.devnet.solana.com");
//
//     let github_account = "ChiefAnthony";
//     let github_bytes = github_account.as_bytes().to_vec();
//
//     let prereq =
//         WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
//
//     let args = CompleteArgs {
//         github: github_bytes,
//     };
//
//     let recent_blockhash = rpc_client
//         .get_latest_blockhash()
//         .expect("Failed to get recent blockhash");
//
//     let transaction = WbaPrereqProgram::complete(
//         &[&signer.pubkey(), &prereq, &system_program::id()],
//         &args,
//         Some(&signer.pubkey()),
//         &[&signer],
//         recent_blockhash,
//     );
//
//     match rpc_client.send_and_confirm_transaction(&transaction) {
//         Ok(signature) => println!(
//             "Success! Check out your TX here:
// https://explorer.solana.com/tx/{}/?cluster=devnet",
//             signature
//         ),
//         Err(e) => println!("Transaction failed: {}", e),
//     }
// }
