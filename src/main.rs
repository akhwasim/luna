mod shell;
mod commands;
mod ai;

#[tokio::main]
async fn main() {
    shell::run();
}