use regex::Regex;

use crate::crane_movers::commands::move_command::MoveCrateCommand;

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

pub fn process_input_lines(lines: &Vec<&str>) -> (Vec<Vec<String>>, Vec<MoveCrateCommand>) {
    let mut item_rows: Vec<Vec<String>> = vec![];
    let mut commands: Vec<MoveCrateCommand> = vec![];
    for line in lines {
        if line.contains("[") {
            let line_data = parse_crate_line(line);
            item_rows.push(line_data);
        } else if line.contains("move") {
            let cmd = parse_command_line(line);
            let crane_quantity = *cmd.get(0).unwrap();
            let origin_stack_position = *cmd.get(1).unwrap();
            let target_stack_position = *cmd.get(2).unwrap();

            commands.push(MoveCrateCommand::new(
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

#[cfg(test)]
mod tests {
    use super::*;

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
