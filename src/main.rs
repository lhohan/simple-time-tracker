use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Time tracking application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    time_tracker::run(&args.input).map_err(anyhow::Error::from)?;
    Ok(())
}
