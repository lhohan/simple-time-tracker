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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let data_path = if let Some(path) = args.input {
        Some(path)
    } else {
        env::current_dir()
            .ok()
            .and_then(|dir| {
                let default_path = dir.join("data/time-entries.md");
                if default_path.exists() {
                    Some(default_path)
                } else {
                    None
                }
            })
    };

    if let Some(ref path) = data_path {
        println!("Starting Time Tracker Web Dashboard...");
        println!("Reading data from: {}", path.display());
        println!("Server running at http://127.0.0.1:3000");
    } else {
        println!("Starting Time Tracker Web Dashboard...");
        println!("⚠️  No data file configured - showing example data");
        println!("Server running at http://127.0.0.1:3000");
        println!();
        println!("To use your data, run:");
        println!("  cargo run --bin tt-web -- -i /path/to/your/data.md");
        println!("  or: just web -i /path/to/your/data.md");
    }

    let state = Arc::new(AppState { data_path });
    let app = web::server::create_router_with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
