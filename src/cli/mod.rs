use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Simple time tgit reracking from markdown files"
)]
pub struct Args {
    /// Input file to process
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }
}
