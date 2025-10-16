use anyhow::Result;
use chrono::NaiveDate;
use time_tracker::cli::Args;
use time_tracker::domain::time::Clock;

fn main() -> Result<()> {
    let args = Args::parse()
        .map_err(|err| anyhow::anyhow!("Error parsing command line arguments: {err}"))?;
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

    let filter = args.context_filter();
    let exclude_tags = args.exclude_tags();
    let period = args.period(&clock)?;
    let formatter = args.formatter();
    let breakdown_unit = args.breakdown_unit(period.as_ref());

    time_tracker::run(
        &args.input,
        args.include_details(),
        filter.as_ref(),
        &exclude_tags,
        period.as_ref(),
        args.limit().as_ref(),
        &*formatter,
        breakdown_unit,
    )
    .map_err(anyhow::Error::from)?;
    Ok(())
}
