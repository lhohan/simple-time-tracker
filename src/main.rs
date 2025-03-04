use anyhow::Result;
use chrono::NaiveDate;
use time_tracker::cli::Args;
use time_tracker::domain::time::Clock;

fn main() -> Result<()> {
    let args = Args::parse();
    let today_str = std::env::var("TT_TODAY").ok();
    let clock = match today_str {
        Some(today_str) => {
            let parsed_date = NaiveDate::parse_from_str(&today_str, "%Y-%m-%d").map_err(|err| {
                anyhow::anyhow!("Error parsing TT_TODAY environment variable: {}", err)
            })?;
            Clock::with_today(parsed_date)
        }
        None => Clock::system(),
    };

    if args.verbose {
        println!("Processing path: {}", args.input.display());
    }

    let exclude_tags = args.exclude_tags();
    let from_date = args.from_date()?;
    let period = args.period(&clock)?;

    time_tracker::run(
        &args.input,
        args.project.as_deref().map(String::from),
        exclude_tags,
        from_date,
        period,
    )
    .map_err(anyhow::Error::from)?;
    Ok(())
}
