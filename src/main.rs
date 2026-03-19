mod db;
mod parser;
mod scheduler;
mod robots;
mod config;
mod crawler;

#[tokio::main]
async fn main() {
    crawler::start().await;
}
