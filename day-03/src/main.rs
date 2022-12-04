use std::fs;

use day_03::{get_sum_of_shared_item_priorities, get_sum_of_badge_priorities};

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    println!("Part 1: {}", get_sum_of_shared_item_priorities(&contents));
    println!("Part 2: {}", get_sum_of_badge_priorities(&contents));
}
