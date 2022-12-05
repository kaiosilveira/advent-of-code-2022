use std::fs;

use day_05::{part_01, part_02};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();

    println!("Result: {}", part_01(&contents));
    println!("Result: {}", part_02(&contents));
}
