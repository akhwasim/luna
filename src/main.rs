mod shell;
mod commands;
mod ai;
mod memory;
mod safety;
mod learner;

#[tokio::main]
async fn main() {
    shell::run();
}