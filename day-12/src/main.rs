use day_12::{distance, parse_input};
use std::fs;

fn main() {
    let input = fs::read_to_string("sample.txt").expect("should be able to read the file");
    let (maze, start, end) = parse_input(&input);
    let pt1_result = distance(&maze, start, end).unwrap();
    println!("part 1: {}", pt1_result);

    let mut pt2_result = pt1_result;
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                if let Some(x) = distance(&maze, (x as isize, y as isize), end) {
                    if pt2_result > x {
                        pt2_result = x;
                    }
                }
            }
        }
    }

    println!("part 2: {}", pt2_result);
}
