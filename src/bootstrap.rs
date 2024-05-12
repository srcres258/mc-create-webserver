use std::env;
use std::error::Error;
use clap::Parser;
use crate::app;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the config file. Using CONFIG_FILE environment variable will override this option
    #[arg(short, long, default_value_t = String::from("config.json"))]
    config: String
}

impl Args {
    pub fn config(&self) -> &String {
        &self.config
    }

    pub fn set_config(&mut self, config: String) {
        self.config = config;
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config: String::from("config.json")
        }
    }
}

pub async fn exec() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();

    if let Ok(config) = env::var("CONFIG_FILE") {
        args.set_config(config);
    }

    app::init(&args).await?;
    app::run().await?;
    app::exit().await;

    Ok(())
}
