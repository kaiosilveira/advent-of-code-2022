use std::cmp::Ordering;

#[derive(Debug)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub fn diff(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn parse_direction(d: &str) -> Direction {
    match d {
        "L" => Direction::LEFT,
        "R" => Direction::RIGHT,
        "U" => Direction::UP,
        "D" => Direction::DOWN,
        _ => panic!("Invalid direction"),
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Part {
    position: Point,
    pub name: String,
    unique_visited_positions: Vec<Point>,
}

impl Part {
    fn new(name: String) -> Part {
        Part {
            name,
            position: Point::new(0, 0),
            unique_visited_positions: vec![Point::new(0, 0)],
        }
    }

    pub fn move_to(&mut self, p: &Point) {
        self.position = p.clone();

        if !self
            .unique_visited_positions
            .iter()
            .any(|point| point.x == p.x && point.y == p.y)
        {
            self.unique_visited_positions.push(p.clone());
        }

        println!("moved to ({}, {})", self.position.x, self.position.y);
    }

    pub fn follow(&mut self, point: &Point) {
        if diff(point.x, self.position.x) < 2 && diff(point.y, self.position.y) < 2 {
            return;
        }

        let new_x = match point.x.cmp(&self.position.x) {
            Ordering::Equal => point.x,
            Ordering::Less => self.position.x - 1,
            Ordering::Greater => self.position.x + 1,
        };

        let new_y = match point.y.cmp(&self.position.y) {
            Ordering::Equal => point.y,
            Ordering::Less => self.position.y - 1,
            Ordering::Greater => self.position.y + 1,
        };

        println!("followed, now at ({}, {})", new_x, new_y);
        self.move_to(&Point::new(new_x, new_y));
    }

    pub fn move_towards(&mut self, direction: &Direction) {
        match direction {
            Direction::LEFT => self.move_to(&Point::new(self.position.x - 1, self.position.y)),
            Direction::RIGHT => self.move_to(&Point::new(self.position.x + 1, self.position.y)),
            Direction::UP => self.move_to(&Point::new(self.position.x, self.position.y + 1)),
            Direction::DOWN => self.move_to(&Point::new(self.position.x, self.position.y - 1)),
        }
    }
}

pub fn part_one(input: &Vec<&str>) -> usize {
    let mut head = Part::new(String::from("head"));
    let mut tail = Part::new(String::from("tail"));

    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parse_direction(parts.get(0).unwrap());
        let number_of_moves = parts.get(1).unwrap().parse::<usize>().unwrap();

        (0..number_of_moves).for_each(|_| {
            head.move_towards(&direction);
            tail.follow(&head.position);
        })
    }

    let total = tail.unique_visited_positions.len();
    println!("total: {}", total);

    total
}

pub fn part_two(input: &Vec<&str>) -> usize {
    let mut head = Part::new(String::from("head"));
    let mut tail1 = Part::new(String::from("1"));
    let mut tail2 = Part::new(String::from("2"));
    let mut tail3 = Part::new(String::from("3"));
    let mut tail4 = Part::new(String::from("4"));
    let mut tail5 = Part::new(String::from("5"));
    let mut tail6 = Part::new(String::from("6"));
    let mut tail7 = Part::new(String::from("7"));
    let mut tail8 = Part::new(String::from("8"));
    let mut tail9 = Part::new(String::from("9"));

    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parse_direction(parts.get(0).unwrap());
        let number_of_moves = parts.get(1).unwrap().parse::<usize>().unwrap();

        (0..number_of_moves).for_each(|_| {
            head.move_towards(&direction);
            tail1.follow(&head.position);
            tail2.follow(&tail1.position);
            tail3.follow(&tail2.position);
            tail4.follow(&tail3.position);
            tail5.follow(&tail4.position);
            tail6.follow(&tail5.position);
            tail7.follow(&tail6.position);
            tail8.follow(&tail7.position);
            tail9.follow(&tail8.position);
        })
    }

    let total = tail9.unique_visited_positions.len();
    println!("total: {}", total);

    total
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_harness_for_part_one() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        assert_eq!(13, part_one(&input));
    }

    #[test]
    fn test_harness_for_part_two() {
        let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
        let larger_input = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];
        assert_eq!(1, part_two(&input));
        assert_eq!(36, part_two(&larger_input));
    }
}
