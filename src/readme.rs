use std::collections::HashMap;


// ███░░░
pub fn hashmap_to_md(stats_map: Vec<(&'static str, i32)>) -> String {
    let mut md_string = String::from("| Month | Contributions | Progress | \n");
    md_string.push_str("|-------|---------------|---------------------------|\n");
    for (month, amount) in stats_map {
        let month_ratio = blocks_ration(amount);
        let md_str = month_block_visual(month_ratio.round() as i32);
        let mut stringify_amount = amount.to_string();
        if(amount<100 && amount>10){
            stringify_amount.push_str(" ")
        } else if(amount<10){
            stringify_amount.push_str("  ")
        }
        let mut result = format!("|{month}|{stringify_amount}");
        result.push_str(&format!("|{}|\n", md_str));

        md_string.push_str(&result);
    }

    md_string
}

fn month_block_visual(blocks: i32)-> String{
    let mut result_str = String::new();
    let max_block_count = 20;
    for _ in 0..blocks {
        result_str.push('█');
    }

    for _ in 0..(max_block_count - blocks) {
        result_str.push('░');
    }

    result_str
}



fn blocks_ration(amount: i32) -> f32 {
    let one_block_amount = 10.0;

    (amount as f32 / one_block_amount)
}