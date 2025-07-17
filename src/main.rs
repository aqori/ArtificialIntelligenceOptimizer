// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizer
 */

use clap::Parser;
use artificialintelligenceoptimizer::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizer - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
