use day_01::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let report_data = input.lines().collect();

    let take_top_n = 3;

    let max_calories = find_top_n_elves_carrying_more_calories(report_data, take_top_n);

    println!("The max calories are {}", max_calories);
}
