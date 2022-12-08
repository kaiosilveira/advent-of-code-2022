use std::fs;

use day_08::get_tree_grid_info;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();

    println!("number of visible trees: {}", part_one(&contents));
    println!("highest scene score: {}", part_two(&contents));
}

pub fn part_one(input: &Vec<&str>) -> usize {
    let (_, number_of_visible_trees) = get_tree_grid_info(&input);
    number_of_visible_trees
}

pub fn part_two(input: &Vec<&str>) -> usize {
    let (highest_scenic_score, _) = get_tree_grid_info(&input);
    highest_scenic_score
}
