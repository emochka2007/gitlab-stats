mod gitlab_api;
mod date;
mod readme;

use gitlab_api::fetch_gitlab_stats;
use tokio;
use date::sort_by_month;
use readme::hashmap_to_md;


#[tokio::main]
async fn main() {
    let stats = fetch_gitlab_stats(String::from("emo")).await.expect("TODO: panic message");

    let sorted_stats = sort_by_month(stats);
    let md = hashmap_to_md(sorted_stats);
    // println!("{:?}", sorted_stats);
}
