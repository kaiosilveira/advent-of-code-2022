use std::fs;

use day_03::{part_1, part_2};

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
