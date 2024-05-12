mod data;
mod messaging;
mod database;
mod app;
mod constants;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    app::init().await?;
    app::run().await?;
    app::exit().await;
    
    Ok(())
}
