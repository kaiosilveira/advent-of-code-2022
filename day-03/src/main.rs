use std::fs;

use day_03::part_2;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    part_2(contents);
}
