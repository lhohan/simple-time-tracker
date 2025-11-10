use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use time_tracker::web::{self, AppState};

#[derive(Parser, Debug)]
#[command(author, version, about = "Time Tracker Web Dashboard")]
struct Args {
    /// Input file to process
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    /// Port to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Host address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

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
        println!("  cargo run --bin tt-web -- -i /path/to/your/data.md");
        println!("  or: just web -i /path/to/your/data.md");
    }

    let state = Arc::new(AppState { data_path });
    let app = web::server::create_router_with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        });

    println!("Press Ctrl+C to shut down");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        });

    println!("Server shut down gracefully");
}

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
