use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Time tracking application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Processing file: {}", args.input.display());
    }

    time_tracker::run(&args.input).map_err(anyhow::Error::from)?;
    Ok(())
}
