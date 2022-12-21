# Grove Positioning System

It's finally time to meet back up with the Elves. When you try to contact them, however, you get no reply. Perhaps you're out of range?

You know they're headed to the grove where the **star** fruit grows, so if you can figure out where that is, you should be able to meet back up with them.

Fortunately, your handheld device has a file (your puzzle input) that contains the grove's coordinates! Unfortunately, the file is **encrypted** - just in case the device were to fall into the wrong hands.

Maybe you can decrypt it?

When you were still back at the camp, you overheard some Elves talking about coordinate file encryption. The main operation involved in decrypting the file is called **mixing**.

The encrypted file is a list of numbers. To **mix** the file, move each number forward or backward in the file a number of positions equal to the value of the number being moved. The list is **circular**, so moving a number off one end of the list wraps back around to the other end as if the ends were connected.

For example, to move the `1` in a sequence like `4, 5, 6, 1, 7, 8, 9` the `1` moves one position forward: `4, 5, 6, 7, 1, 8, 9`. To move the `-2` in a sequence like `4, -2, 5, 6, 7, 8, 9`, the `-2` moves two positions backward, wrapping around: `4, 5, 6, 7, 8, -2, 9`.

The numbers should be moved **in the order they originally appear** in the encrypted file. Numbers moving around during the mixing process do not change the order in which the numbers are moved.

Consider this encrypted file:

```
1
2
-3
3
-2
0
4
```

Mixing this file proceeds as follows:

```
Initial arrangement:
1, 2, -3, 3, -2, 0, 4

1 moves between 2 and -3:
2, 1, -3, 3, -2, 0, 4

2 moves between -3 and 3:
1, -3, 2, 3, -2, 0, 4

-3 moves between -2 and 0:
1, 2, 3, -2, -3, 0, 4

3 moves between 0 and 4:
1, 2, -2, -3, 0, 3, 4

-2 moves between 4 and 1:
1, 2, -3, 0, 3, 4, -2

0 does not move:
1, 2, -3, 0, 3, 4, -2

4 moves between -3 and 0:
1, 2, -3, 4, 0, 3, -2
```

Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers after the value 0, wrapping around the list as necessary. In the above example, the 1000th number after `0` is **4**, the 2000th is **-3**, and the 3000th is **2**; adding these together produces **3**.

Mix your encrypted file exactly once. **What is the sum of the three numbers that form the grove coordinates?**

## Part I: part one title

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
pub fn part_one(_input: &Vec<&str>) {
  todo!();
}
```

</details>

---

## Part II: part two title

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
