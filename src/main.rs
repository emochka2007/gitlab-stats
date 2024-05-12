mod gitlab_api;
mod date;
mod readme;

use gitlab_api::fetch_gitlab_stats;
use tokio;
use date::sort_by_month;
use readme::hashmap_to_md;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <username>");
        return;
    }
    let username = &args[1];
    let stats = fetch_gitlab_stats(username.clone()).await.expect("Failed fetching stats");

    let sorted_stats = sort_by_month(stats);
    let md = hashmap_to_md(sorted_stats);
    println!("{}", md);
}
