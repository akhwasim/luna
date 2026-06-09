mod shell;
mod commands;
mod ai;
mod memory;
mod safety;
mod learner;
mod stats;

#[tokio::main]
async fn main() {
    shell::run();
}