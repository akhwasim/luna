mod shell;
mod commands;
mod ai;
mod memory;
mod safety;


#[tokio::main]
async fn main() {
    shell::run();
}