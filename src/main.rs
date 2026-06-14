mod shell;
mod commands;
mod ai;
mod memory;
mod safety;
mod learner;
mod stats;
mod config;
mod setup;

#[tokio::main]
async fn main() {
    setup::run().await;
}