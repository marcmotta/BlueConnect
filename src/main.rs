// src/main.rs
/*
 * Main executable for BlueConnect
 */

use clap::Parser;
use blueconnect::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlueConnect - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
