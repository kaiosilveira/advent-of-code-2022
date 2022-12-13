# Hill Climbing Algorithm

You try contacting the Elves using your handheld device, but the river you're following must be too low to get a decent signal.

You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap shows the local area from above broken into a grid; the elevation of each square of the grid is given by a single lowercase letter, where `a` is the lowest elevation, `b` is the next-lowest, and so on up to the highest elevation, `z`.

Also included on the heightmap are marks for your current position (`S`) and the location that should get the best signal (`E`). Your current position (`S`) has elevation a, and the location that should get the best signal (`E`) has elevation `z`.

## Part I: Finding the shortest path to the top

You'd like to reach `E`, but to save energy, you should do it in **as few steps as possible**. During each step, you can move exactly one square up, down, left, or right. To avoid needing to get out your climbing gear, the elevation of the destination square can be **at most one higher** than the elevation of your current square; that is, if your current elevation is m, you could step to elevation n, but not to elevation o. (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)

For example:

```
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
```

Here, you start in the top-left corner; your goal is near the middle. You could start by moving down or right, but eventually you'll need to head toward the e at the bottom. From there, you can spiral around to the goal:

```
v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^
```

In the above diagram, the symbols indicate whether the path exits each square moving up (`^`), down (`v`), left (`<`), or right (`>`). The location that should get the best signal is still `E`, and `.` marks unvisited squares.

This path reaches the goal in `31` steps, the fewest possible.

**What is the fewest steps required to move from your current position to the location that should get the best signal?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

In a test-driven development fashion, we can use the example provided in the text above as a test harness, to make sure that, for a given input, we're returning the correct output. We can make this test pass as fast as possible just by hard-coding its return value, so we're always on the green bar, and as soon we're able to replace the hard-coded values by real code, it means we're finished. The test harness extracted from the example is:

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn test_harness_for_part_one() {}
}
```

### Solution

```rust
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
```

</details>

---

## Part II: Finding the ideal place to start

As you walk up the hill, you suspect that the Elves will want to turn this into a hiking trail. The beginning isn't very scenic, though; perhaps you can find a better starting point.

To maximize exercise while hiking, the trail should start as low as possible: elevation a. The goal is still the square marked E. However, the trail should still be direct, taking the fewest steps to reach its goal. So, you'll need to find the shortest path from **any square at elevation a** to the square marked `E`.

Again consider the example from above:

```
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
```

Now, there are six choices for starting position (five marked a, plus the square marked S that counts as being at elevation a). If you start at the bottom-left square, you can reach the goal most quickly:

```
...v<<<<
...vv<<^
...v>E^^
.>v>>>^^
>^>>>>>^
```

This path reaches the goal in only `29` steps, the fewest possible.

**What is the fewest steps required to move starting from any square with elevation a to the location that should get the best signal?**

<details>
<summary><strong>🚧 WIP: See solution 🚧</strong></summary>

Similar to what we did for part one, the test harness for part two is:

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn test_harness_for_part_two() {}
}
```

### Solution

```rust
pub fn part_two(_input: &Vec<&str>) {
  todo!();
}
```

</details>
