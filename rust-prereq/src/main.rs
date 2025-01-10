use rust_prereq::submit_completion;

mod keygen;
mod register;
mod transfer;

#[tokio::main]
async fn main() {
    submit_completion().await;
}
