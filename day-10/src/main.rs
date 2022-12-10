use std::fs;

use day_10::{part_one, part_two};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();

    part_one(&contents);
    part_two(&contents);
}
