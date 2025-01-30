use anyhow::Result;
use time_tracker::cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Processing path: {}", args.input.display());
    }

    let from_date = args.from_date()?;

    time_tracker::run(&args.input, args.project.as_deref(), from_date)
        .map_err(anyhow::Error::from)?;
    Ok(())
}
