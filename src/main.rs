use std::error::Error;

mod data;
mod messaging;
mod database;
mod app;
mod constants;
mod bootstrap;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    bootstrap::exec().await
}
