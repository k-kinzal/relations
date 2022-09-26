mod commands;
mod configs;
mod databases;

use crate::commands::{run_generate, Cli, Commands};
use anyhow::Result;
use clap::Parser;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        })
        .init();

    match args.command {
        Commands::Generate { command } => run_generate(command).await,
    }
}
