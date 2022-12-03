use day_02::strategies::{
    rock_paper_scissors_guessed_strategy::*, rock_paper_scissors_real_strategy::*,
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the input file");
    let contents: Vec<&str> = contents.lines().collect();

    run_real_strategy(&contents);
    println!("-------------------");
    run_guessed_strategy(&contents);
}
