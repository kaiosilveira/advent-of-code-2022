use std::fs;

use day_15::part_one;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    println!("part I: {}", part_one(&contents));
}
