use std::fs;

use day_07::{find_dir_to_delete, find_total_sizes_of_directories};

fn main() {
    let input = fs::read_to_string("sample.txt").expect("Should be able to read the file");
    let input: Vec<&str> = input.lines().collect();

    println!(
        "Sum of all directories with size of at most 100000: {}",
        find_total_sizes_of_directories(&input)
    );

    println!(
        "Size of the directory to delete so we can process the update: {}",
        find_dir_to_delete(&input)
    );
}
