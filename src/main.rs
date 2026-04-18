mod shell;
mod commands;
mod ai;
mod memory;

#[tokio::main]
async fn main() {
    shell::run();
}