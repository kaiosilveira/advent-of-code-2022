use std::fs;

use day_05::{move_crates_one_by_one, move_multiple_crates_at_once};

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();

    println!("Part 1: {}", move_crates_one_by_one(&contents));
    println!("Part 2: {}", move_multiple_crates_at_once(&contents));
}
