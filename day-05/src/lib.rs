mod crane_movers;
mod stacks;
mod utils;

use crane_movers::{
    cranes::{crate_mover_9000::CrateMover9000, crate_mover_9001::CrateMover9001},
    strategies::move_crate_strategy::MoveCrateStrategy,
};
use stacks::crate_stack::CrateStack;
use utils::input_parsing::{get_number_of_columns_from, process_input_lines};

pub fn move_crates_one_by_one(input_lines: &Vec<&str>) -> String {
    process_commands_and_return_the_topmost_item(&CrateMover9000::new(), input_lines)
}

pub fn move_multiple_crates_at_once(input_lines: &Vec<&str>) -> String {
    process_commands_and_return_the_topmost_item(&CrateMover9001::new(), input_lines)
}

pub fn get_topmost_item_from_each_stack(stacks: &Vec<CrateStack>) -> String {
    let mut items: Vec<&str> = vec![];
    for c in stacks {
        items.push(c.first());
    }

    items
        .iter()
        .map(|s| s.replace("[", "").replace("]", ""))
        .collect::<Vec<String>>()
        .join("")
}

pub fn process_commands_and_return_the_topmost_item(
    crane: &impl MoveCrateStrategy,
    input_lines: &Vec<&str>,
) -> String {
    let (item_rows, commands) = process_input_lines(input_lines);
    let mut stacks = CrateStack::from_rows(&item_rows);

    for command in commands {
        command.apply_using(crane, &mut stacks);
    }

    get_topmost_item_from_each_stack(&stacks)
}

pub fn print_stacks(stacks: &Vec<Vec<String>>) {
    println!("");
    (0..stacks.len()).for_each(|n| {
        for c in stacks {
            match c.get(n) {
                Some(v) => print!("{:?}", v),
                None => print!("     "),
            }
        }
        println!("");
    });
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = vec![
            "    [D]   ",
            "[N] [C]   ",
            "[Z] [M] [P]",
            " 1   2   3",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let r = move_crates_one_by_one(&contents);
        assert_eq!("CMZ", r);
    }

    #[test]
    fn test_part_2() {
        let contents = vec![
            "    [D]   ",
            "[N] [C]   ",
            "[Z] [M] [P]",
            " 1   2   3",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let r = move_multiple_crates_at_once(&contents);
        assert_eq!("MCD", r);
    }
}
