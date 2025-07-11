// src/main.rs
/*
 * Main executable for BlockchainNftRegistry
 */

use clap::Parser;
use blockchainnftregistry::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNftRegistry - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
