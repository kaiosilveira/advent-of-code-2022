use std::ops::Range;

const LIT_PIXEL: &str = "#";
const DARK_PIXEL: &str = ".";

#[derive(Debug)]
pub struct CPUInstruction {
    finishes_at: usize,
    value_to_add: i32,
}

impl CPUInstruction {
    pub fn new(finishes_at: usize, should_add: i32) -> CPUInstruction {
        CPUInstruction {
            finishes_at,
            value_to_add: should_add,
        }
    }
}

pub struct Sprite {
    pub current_position: Range<i32>,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            current_position: 0..3,
        }
    }

    pub fn update(&mut self, x: &i32) {
        let start = x - 1;
        let end = x + 2;

        self.current_position = start..end;
    }
}

#[derive(Clone)]
pub struct CRTScreen {
    pub rows: Vec<Vec<String>>,
}

impl CRTScreen {
    pub fn new() -> CRTScreen {
        let create_row = || -> Vec<String> {
            "........................................"
                .split("")
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        };

        CRTScreen {
            rows: vec![
                create_row(),
                create_row(),
                create_row(),
                create_row(),
                create_row(),
                create_row(),
            ],
        }
    }

    pub fn draw(&mut self, cycle: &usize, sprite: &Sprite) {
        let crt_row_idx = (cycle - 1) / 40;
        let crt_row = self.rows.get_mut(crt_row_idx).unwrap();
        let row_idx = (cycle - 1).rem_euclid(40) as usize;

        crt_row[row_idx] = match &sprite.current_position {
            p if p.contains(&(row_idx as i32)) => String::from(LIT_PIXEL),
            _ => String::from(DARK_PIXEL),
        };
    }

    pub fn render(&self) {
        for row in &self.rows {
            println!("{}", row.join(""));
        }
    }
}

pub fn parse_cpu_instructions(input: &Vec<&str>) -> Vec<CPUInstruction> {
    let mut pending_work: Vec<CPUInstruction> = vec![];
    let mut next_available_cycle: usize = 1;

    for line in input {
        match line {
            l if l.contains("add") => {
                let parts: Vec<&str> = l.split(" ").collect();
                let adder = parts.get(1).unwrap().parse::<i32>().unwrap();
                pending_work.push(CPUInstruction::new(next_available_cycle + 1, adder));
                next_available_cycle += 2;
            }
            l if l.contains("noop") => {
                pending_work.push(CPUInstruction::new(next_available_cycle, 0));
                next_available_cycle += 1;
            }
            _ => panic!("Invalid input"),
        }
    }

    pending_work
}

pub fn process_cpu_instructions<'a>(input: &Vec<&'a str>) -> (Vec<i32>, CRTScreen) {
    let mut crt_screen = CRTScreen::new();
    let mut sprite = Sprite::new();
    let mut x: i32 = 1;

    let mut signal_lengths: Vec<i32> = vec![];
    signal_lengths.push(0);

    let pending_work = parse_cpu_instructions(input);

    let last_scheduled_cmd = pending_work.last().unwrap().finishes_at;
    (1..=last_scheduled_cmd).for_each(|cycle| {
        crt_screen.draw(&cycle, &sprite);

        let signal_length = x * cycle as i32;
        signal_lengths.push(signal_length);

        let instruction = &pending_work.iter().find(|i| i.finishes_at == cycle);
        match instruction {
            Some(i) => x += i.value_to_add,
            None => (),
        }

        sprite.update(&x);
    });

    (signal_lengths, crt_screen)
}

pub fn part_one(input: &Vec<&str>) -> i32 {
    let (lengths, _) = process_cpu_instructions(input);
    let selected_signal_lengths = (20..=220)
        .step_by(40)
        .map(|i| lengths.get(i).unwrap_or(&0i32));

    let total = selected_signal_lengths.fold(0, |a, b| a + b);

    println!("Sum of selected signal lengths: {}", total);
    total
}

pub fn part_two(input: &Vec<&str>) -> Vec<Vec<String>> {
    let (_, screen) = process_cpu_instructions(input);
    screen.render();
    screen.rows
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_harness_for_part_one() {
        let larger_input = vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];

        assert_eq!(13140, part_one(&larger_input));
    }

    #[test]
    fn test_harness_for_part_two() {
        let larger_input = vec![
            "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
            "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5",
            "addx -1", "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1",
            "addx 16", "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop",
            "addx -3", "addx 9", "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop",
            "noop", "noop", "noop", "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop",
            "addx 2", "addx 6", "noop", "noop", "noop", "noop", "noop", "addx 1", "noop", "noop",
            "addx 7", "addx 1", "noop", "addx -13", "addx 13", "addx 7", "noop", "addx 1",
            "addx -33", "noop", "noop", "noop", "addx 2", "noop", "noop", "noop", "addx 8", "noop",
            "addx -1", "addx 2", "addx 1", "noop", "addx 17", "addx -9", "addx 1", "addx 1",
            "addx -3", "addx 11", "noop", "noop", "addx 1", "noop", "addx 1", "noop", "noop",
            "addx -13", "addx -19", "addx 1", "addx 3", "addx 26", "addx -30", "addx 12",
            "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9", "addx 18", "addx 1",
            "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1", "addx 2",
            "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22", "addx -6",
            "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop", "addx 20",
            "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
        ];

        let screen = part_two(&larger_input);
        assert_eq!(
            "##..##..##..##..##..##..##..##..##..##...",
            screen.get(0).unwrap().join("")
        );
        assert_eq!(
            "###...###...###...###...###...###...###..",
            screen.get(1).unwrap().join("")
        );
        assert_eq!(
            "####....####....####....####....####.....",
            screen.get(2).unwrap().join("")
        );
        assert_eq!(
            "#####.....#####.....#####.....#####......",
            screen.get(3).unwrap().join("")
        );
        assert_eq!(
            "######......######......######......####.",
            screen.get(4).unwrap().join("")
        );
        assert_eq!(
            "#######.......#######.......#######......",
            screen.get(5).unwrap().join("")
        );
    }
}
