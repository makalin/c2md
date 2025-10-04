use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;

mod cli;
mod config;
mod converter;
mod error;
mod utils;
mod validator;
mod metadata;
mod template;

use cli::Args;
use config::Config;
use converter::Converter;
use error::C2mdError;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    
    // Set up logging level based on verbosity
    if args.verbose {
        std::env::set_var("RUST_LOG", "debug");
    }

    info!("Starting c2md v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::load(&args.config)?;
    
    // Create converter instance
    let converter = Converter::new(config, args.clone())?;

    // Handle different modes
    match converter.process().await {
        Ok(()) => {
            info!("Conversion completed successfully");
            std::process::exit(0);
        }
        Err(e) => {
            error!("Conversion failed: {}", e);
            std::process::exit(1);
        }
    }
}