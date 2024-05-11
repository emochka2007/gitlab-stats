use std::collections::HashMap;
use chrono::{NaiveDate, Datelike, Utc};

const MONTHS: [&str; 12] = [
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN",
    "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"
];
pub fn sort_by_month(stats: HashMap<String, i32>)-> Vec<(&'static str, i32)> {
    let current_month = Utc::now().month() as usize;
    let current_month_index = current_month - 1;

    let mut results: Vec<(&str, i32)> = MONTHS.iter().map(|&m| (m, 0)).collect();

    for (date, amount) in stats.iter() {
        let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .expect("Invalid date format");
        let month_index = parsed_date.month() as usize - 1;
        results[month_index].1 += amount;
    }

    let mut sorted_results = Vec::new();
    for i in 0..12 {
        let idx = (current_month_index + i) % 12;
        sorted_results.push(results[idx].clone());
    }

    sorted_results
}