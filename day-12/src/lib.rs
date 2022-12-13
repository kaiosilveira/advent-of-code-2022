use std::collections::HashSet;
use std::collections::VecDeque;

type Step = (isize, isize);
type Maze = Vec<Vec<u8>>;

pub fn parse_input(input: &str) -> (Maze, (isize, isize), (isize, isize)) {
    let mut maze = Maze::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in input.lines().enumerate() {
        let mut r = Vec::new();
        for (j, cell) in row.chars().enumerate() {
            r.push(if cell == 'S' {
                start = (j as isize, i as isize);
                0
            } else if cell == 'E' {
                end = (j as isize, i as isize);
                25
            } else {
                (cell as u8) - 97
            });
        }
        maze.push(r);
    }

    (maze, start, end)
}

pub fn distance(m: &Maze, start: Step, end: Step) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0, start));
    visited.insert(start);
    while let Some((distance, (x, y))) = queue.pop_front() {
        for (nx, ny) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            let unx = nx as usize;
            let uny = ny as usize;
            if nx >= 0
                && ny >= 0
                && uny < m.len()
                && unx < m[uny].len()
                && m[uny][unx] <= m[y as usize][x as usize] + 1
                && visited.insert((nx, ny))
            {
                if (nx, ny) == end {
                    return Some(distance + 1);
                }
                queue.push_back((distance + 1, (nx, ny)));
            }
        }
    }
    None
}
