use time_tracker::web;

#[tokio::main]
async fn main() {
    println!("Starting Time Tracker Web Dashboard...");
    println!("Server running at http://127.0.0.1:3000");
    web::run_server("127.0.0.1:3000").await;
}
