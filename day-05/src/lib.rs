mod crane_movers;

use crane_movers::cranes::{CraneMover9000, CraneMover9001, CraneMoverCommand, MoveCraneStrategy};
use regex::Regex;

pub fn parse_crate_line(line: &str) -> Vec<String> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .map(|s| String::from(s.trim()))
        .collect::<Vec<String>>()
}

pub fn parse_command_line(line: &str) -> Vec<usize> {
    let mut cmd: Vec<usize> = vec![];

    for cap in Regex::new(r"\w+\s\d+\s?").unwrap().captures_iter(line) {
        let cmd_part: Vec<&str> = cap.get(0).unwrap().as_str().trim().split(" ").collect();
        cmd.push(cmd_part.get(1).unwrap().parse::<usize>().unwrap());
    }

    cmd
}

pub fn process_input_lines(lines: &Vec<&str>) -> (Vec<Vec<String>>, Vec<CraneMoverCommand>) {
    let mut item_rows: Vec<Vec<String>> = vec![];
    let mut commands: Vec<CraneMoverCommand> = vec![];
    for line in lines {
        if line.contains("[") {
            let line_data = parse_crate_line(line);
            item_rows.push(line_data);
        } else if line.contains("move") {
            let cmd = parse_command_line(line);
            let crane_quantity = *cmd.get(0).unwrap();
            let origin_stack_position = *cmd.get(1).unwrap();
            let target_stack_position = *cmd.get(2).unwrap();

            commands.push(CraneMoverCommand::new(
                crane_quantity,
                origin_stack_position,
                target_stack_position,
            ));
        }
    }

    (item_rows, commands)
}

pub fn get_number_of_columns_from(rows: &Vec<Vec<String>>) -> usize {
    rows.iter().map(|r| r.len()).max().unwrap()
}

pub fn create_columns_from_rows(item_rows: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut columns: Vec<Vec<String>> = vec![];
    let len = get_number_of_columns_from(item_rows);

    (0..len).into_iter().for_each(|n| {
        let mut column: Vec<String> = Vec::new();
        for row in item_rows {
            match &row.get(n) {
                Some(v) => {
                    if !v.is_empty() {
                        column.push(v.to_string())
                    }
                }
                None => (),
            }
        }

        columns.push(column.clone());
    });

    columns
}

pub fn apply_commands_to_stacks(
    commands: &Vec<CraneMoverCommand>,
    stacks: &mut Vec<Vec<String>>,
    crate_mover: impl MoveCraneStrategy,
) {
    for cmd in commands {
        crate_mover.process_move_command(cmd, stacks);
        print_stacks(&stacks);
    }
}

pub fn get_topmost_item_from_each_stack(stacks: &Vec<Vec<String>>) -> String {
    let mut items: Vec<&str> = vec![];
    for c in stacks {
        items.push(c.get(0).unwrap());
    }

    items
        .iter()
        .map(|s| s.replace("[", "").replace("]", ""))
        .collect::<Vec<String>>()
        .join("")
}

pub fn move_crates_one_by_one(stacks: &Vec<&str>) -> String {
    let (item_rows, commands) = process_input_lines(stacks);
    let mut stacks = create_columns_from_rows(&item_rows);

    apply_commands_to_stacks(&commands, &mut stacks, CraneMover9000::new());

    get_topmost_item_from_each_stack(&stacks)
}

pub fn move_multiple_crates_at_once(contents: &Vec<&str>) -> String {
    let (item_rows, commands) = process_input_lines(contents);
    let mut stacks = create_columns_from_rows(&item_rows);

    print_stacks(&stacks);

    apply_commands_to_stacks(&commands, &mut stacks, CraneMover9001::new());

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

    mod parse_crate_line_tests {
        use super::*;

        #[test]
        fn should_parse_a_crate_line() {
            let line = "[Z] [M] [P]";

            let result = parse_crate_line(&line);

            assert_eq!("[Z]", result.get(0).unwrap());
            assert_eq!("[M]", result.get(1).unwrap());
            assert_eq!("[P]", result.get(2).unwrap());
        }
    }

    mod parse_command_line_tests {
        use super::*;

        #[test]
        fn should_parse_a_command_line() {
            let line = "move 1 from 2 to 1";

            let result = parse_command_line(&line);

            assert_eq!(1, *result.get(0).unwrap());
            assert_eq!(2, *result.get(1).unwrap());
            assert_eq!(1, *result.get(2).unwrap());
        }
    }
}
