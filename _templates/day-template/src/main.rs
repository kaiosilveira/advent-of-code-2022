use std::fs;

use day_08::part_one;

fn main() {
    let mut contents = fs::read_to_string("sample.txt").expect("Should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();

    part_one(&contents);
    part_one(&contents);
}
