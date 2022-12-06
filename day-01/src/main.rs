use day_01::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("sample.txt").expect("Should be able to read the input file");
    let report_data = input.lines().collect();

    let total_calories = find_the_elf_carrying_more_calories(&report_data);
    let total_top_three_calories = find_top_three_elves_carrying_more_calories(&report_data);

    println!(
        "The sum of calories carried by the elf with more calories is: {}",
        total_calories
    );
    println!(
        "The sum of calories carried by the top three elves with more calories is: {}",
        total_top_three_calories
    );
}
