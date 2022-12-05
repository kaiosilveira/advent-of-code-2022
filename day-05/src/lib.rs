use regex::Regex;

pub fn part_01(contents: &Vec<&str>) -> String {
    let mut item_rows: Vec<Vec<String>> = vec![];
    let mut commands: Vec<Vec<usize>> = vec![];
    for line in contents {
        if line.contains("[") {
            let line_data = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .map(|s| String::from(s.trim()))
                .collect::<Vec<String>>();

            item_rows.push(line_data);
        } else if line.contains("move") {
            let mut cmd: Vec<usize> = vec![];
            for cap in Regex::new(r"\w+\s\d+\s?").unwrap().captures_iter(line) {
                let cmd_part: Vec<&str> = cap.get(0).unwrap().as_str().trim().split(" ").collect();
                cmd.push(cmd_part.get(1).unwrap().parse::<usize>().unwrap());
            }
            commands.push(cmd.clone());
        }
    }

    let len = item_rows.iter().map(|r| r.len()).max().unwrap();
    println!("len: {}", len);
    let mut columns: Vec<Vec<String>> = vec![];

    (0..len).into_iter().for_each(|n| {
        let mut column: Vec<String> = Vec::new();
        for row in &item_rows {
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

    print_columns(&len, &columns);

    for cmd in commands {
        let mv = cmd.get(0).unwrap();
        let from = cmd.get(1).unwrap();
        let to = cmd.get(2).unwrap();

        let origin = columns.get_mut(*from - 1).unwrap();

        let items_to_move: Vec<String> = origin.drain(0..mv.clone()).collect();

        println!(
            "Moving {} items ({:?}) from {} to {}",
            mv, items_to_move, from, to
        );

        let target = columns.get_mut(*to - 1).unwrap();
        for item in items_to_move {
            target.insert(0, item);
        }

        print_columns(&len, &columns);
    }

    let mut items: Vec<&str> = vec![];
    for c in &columns {
        items.push(c.get(0).unwrap());
    }

    items
        .iter()
        .map(|s| s.replace("[", "").replace("]", ""))
        .collect::<Vec<String>>()
        .join("")
}

pub fn part_02(contents: &Vec<&str>) -> String {
    let mut item_rows: Vec<Vec<String>> = vec![];
    let mut commands: Vec<Vec<usize>> = vec![];
    for line in contents {
        if line.contains("[") {
            let line_data = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .map(|s| String::from(s.trim()))
                .collect::<Vec<String>>();

            item_rows.push(line_data);
        } else if line.contains("move") {
            let mut cmd: Vec<usize> = vec![];
            for capture in Regex::new(r"\w+\s\d+\s?").unwrap().captures_iter(line) {
                let cmd_part: Vec<&str> =
                    capture.get(0).unwrap().as_str().trim().split(" ").collect();
                cmd.push(cmd_part.get(1).unwrap().parse::<usize>().unwrap());
            }
            commands.push(cmd.clone());
        }
    }

    let len = item_rows.iter().map(|r| r.len()).max().unwrap();
    println!("len: {}", len);
    let mut columns: Vec<Vec<String>> = vec![];

    (0..len).into_iter().for_each(|n| {
        let mut column: Vec<String> = Vec::new();
        for row in &item_rows {
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

    print_columns(&len, &columns);

    for cmd in commands {
        let mv = cmd.get(0).unwrap();
        let from = cmd.get(1).unwrap();
        let to = cmd.get(2).unwrap();

        let origin = columns.get_mut(*from - 1).unwrap();

        let items_to_move: Vec<String> = origin.drain(0..mv.clone()).collect();

        println!(
            "Moving {} items ({:?}) from {} to {}",
            mv, items_to_move, from, to
        );

        let target = columns.get_mut(*to - 1).unwrap();
        target.splice(0..0, items_to_move);

        print_columns(&len, &columns);
    }

    let mut items: Vec<&str> = vec![];
    for c in &columns {
        items.push(c.get(0).unwrap());
    }

    items
        .iter()
        .map(|s| s.replace("[", "").replace("]", ""))
        .collect::<Vec<String>>()
        .join("")
}

pub fn print_columns(len: &usize, columns: &Vec<Vec<String>>) {
    println!("");
    (0..*len).for_each(|n| {
        for c in columns {
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

        let r = part_01(&contents);
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

        let r = part_02(&contents);
        assert_eq!("MCD", r);
    }
}
