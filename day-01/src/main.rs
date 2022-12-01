use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let report_data = input.lines().collect();

    let take_top_n = 3;

    let max_calories = day_01::find_the_elf_carrying_most_calories(report_data, take_top_n);

    println!("The max calories are {}", max_calories);
}
