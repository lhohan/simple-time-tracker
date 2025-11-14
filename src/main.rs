use anyhow::Result;
use chrono::NaiveDate;
use time_tracker::cli::Args;
use time_tracker::cli::statistics::{StatisticsCollector, write_stat_record};
use time_tracker::domain::time::Clock;

#[cfg(feature = "web")]
use std::env;
#[cfg(feature = "web")]
use std::sync::Arc;

fn main() -> Result<()> {
    let args = Args::parse()
        .map_err(|err| anyhow::anyhow!("Error parsing command line arguments: {err}"))?;

    let stat_record = StatisticsCollector::from_args(&args);

    let result = if args.web {
        run_web_server(args)
    } else {
        run_cli(args)
    };

    match result {
        Ok(()) => {
            let _ = write_stat_record(&stat_record);
            Ok(())
        }
        Err(e) => {
            let mut failed_record = stat_record;
            failed_record = StatisticsCollector::with_failure(failed_record, "execution_error".to_string());
            let _ = write_stat_record(&failed_record);
            Err(e)
        }
    }
}

fn run_cli(args: Args) -> Result<()> {
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

    let input = args.input.as_ref().expect("input is required in CLI mode");

    if args.verbose {
        println!("Processing path: {}", input.display());
    }

    let filter = args.context_filter();
    let exclude_tags = args.exclude_tags();
    let period = args.period(&clock)?;
    let formatter = args.formatter();
    let breakdown_unit = args.breakdown_unit(period.as_ref());

    time_tracker::run(
        input,
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

#[cfg(feature = "web")]
fn run_web_server(args: Args) -> Result<()> {
    use time_tracker::web::{self, AppState};

    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let data_path = if let Some(path) = args.input {
            Some(path)
        } else {
            env::current_dir().ok().and_then(|dir| {
                let default_path = dir.join("data/time-entries.md");
                if default_path.exists() {
                    Some(default_path)
                } else {
                    None
                }
            })
        };

        let addr = format!("{}:{}", args.host, args.port);

        if let Some(ref path) = data_path {
            println!("Starting Time Tracker Web Dashboard...");
            println!("Reading data from: {}", path.display());
            println!("Server running at http://{}", addr);
        } else {
            println!("Starting Time Tracker Web Dashboard...");
            println!("⚠️  No data file configured - showing example data");
            println!("Server running at http://{}", addr);
            println!();
            println!("To use your data, run:");
            println!("  tt --web -i /path/to/your/data.md");
        }

        let state = Arc::new(AppState { data_path });
        let app = web::server::create_router_with_state(state);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

        println!("Press Ctrl+C to shut down");

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

        println!("Server shut down gracefully");
        Ok(())
    })
}

#[cfg(not(feature = "web"))]
fn run_web_server(_args: Args) -> Result<()> {
    anyhow::bail!("Web server feature is not enabled. Rebuild with --features web")
}

#[cfg(feature = "web")]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, starting graceful shutdown...");
}
