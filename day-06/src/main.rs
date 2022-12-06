use std::fs;

use day_06::{find_start_of_message, find_start_of_packet};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read the file");
    println!(
        "Start of the packet transmission: {}",
        find_start_of_packet(&contents)
    );
    println!("Start of the message: {}", find_start_of_message(&contents));
}
