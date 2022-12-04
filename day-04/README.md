# Camp Cleanup

Challenge URL: https://adventofcode.com/2022/day/4

Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.

## Part One: Shared assignment lists

However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).

For example, consider the following list of section assignment pairs:

```
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
```

For the first few pairs, this list means:

- Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
- The Elves in the second pair were each assigned two sections.
- The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.

This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:

```
.234..... 2-4
.....678. 6-8

.23...... 2-3
...45.... 4-5

....567.. 5-7
......789 7-9

.2345678. 2-8
..34567.. 3-7

.....6... 6-6
...456... 4-6

.23456... 2-6
...45678. 4-8
```

Some of the pairs have noticed that one of their assignments fully contains the other. For example, `2-8` fully contains `3-7`, and `6-6` is fully contained by `4-6`. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.

**In how many assignment pairs does one range fully contain the other?**

<details>
<summary><strong>See solution</strong></summary>

Thanks to Rust's `Range` construct, solving this problem is straightforward. We can start by handling input data and parsing it into proper ranges. As each line of input is in the format `2-4,6-8`, we can start small and create a function to convert the task info of the first elf into a range, so `2-4` should become `2..=4`. The code for this is:

```rust
pub fn create_range_of_tasks(task_range_info: &str) -> RangeInclusive<i32> {
    let parts: Vec<&str> = task_range_info.split("-").collect();
    let start: i32 = parts.get(0).unwrap().parse().unwrap();
    let end: i32 = parts.get(1).unwrap().parse().unwrap();

    start..=end
}

#[cfg(test)]
mod tests {
    // -- snip --
    #[test]
    fn should_create_a_range_from_a_string() {
        let range = create_range_of_tasks("2-8");
        let range: Vec<i32> = range.collect();

        assert_eq!(&2, range.first().unwrap());
        assert_eq!(&8, range.last().unwrap());
    }
}
```

We're basically splitting the input string (`2-8` in the test case) into two parts: `2` and `8`, then we are returning a range that starts at `start` (2) and ends at `end` inclusive (8). Error handling was pretty much overlooked here, as we're only using `.unwrap()` that will cause the program to panic if we don't get what we're expecting from the string.

With the function above, now we can parse a full line of input:

```rust
pub fn create_task_ranges_from_pair_info(str: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let parts: Vec<&str> = str.split(",").collect();

    (
        create_range_of_tasks(parts.get(0).unwrap()),
        create_range_of_tasks(parts.get(1).unwrap()),
    )
}

#[cfg(test)]
mod tests {
    // -- snip --
    #[test]
    fn should_create_ranges_from_pair_info() {
        let pair_info = "2-8,3-7";
        let (r1, r2) = create_task_ranges_from_pair_info(pair_info);
        let r1: Vec<i32> = r1.collect();
        let r2: Vec<i32> = r2.collect();

        assert_eq!(&2, r1.first().unwrap());
        assert_eq!(&8, r1.last().unwrap());

        assert_eq!(&3, r2.first().unwrap());
        assert_eq!(&7, r2.last().unwrap());
    }
}
```

The logic here is the same: splitting the line at `,`, then giving the two parts to `create_range_of_tasks`, which knows how to create a range out of a well-formatted string.

Now that we are working with ranges, we just need to implement logic to check whether or not a range fully contains the other:

```rust
pub fn range_fully_contains_other(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    let mut fully_contains = true;

    for n in r2.clone() {
        if !r1.contains(&n) {
            fully_contains = false;
            break;
        }
    }

    fully_contains
}

#[cfg(test)]
mod tests {
    // -- snip --
  #[test]
    fn should_check_whether_or_not_a_range_fully_contains_the_other() {
        assert!(range_fully_contains_other(&(2..=3), &(2..=2)));
        assert!(range_fully_contains_other(&(2..=8), &(3..=7)));
        assert!(!range_fully_contains_other(&(3..=7), &(2..=8)));
    }
}
```

We're basically walking through each item in `r2`, checking whether or not `r1` contains it. As we want all items to be contained in the range, we are optimistically defining `fully_contains` as `true`. Then we iterate over a clone of `r2` and, if `r1` does not contain a given item, we set `fully_contains` to `false` and exit the loop.

That's all we need for part one, now we just need to add boilerplate code to load the input data and print the output:

```rust
fn main() {
    let contents = fs::read_to_string("sample.txt").expect("Should be able to read the file");

    let mut total_fully_contained = 0;
    for line in contents.lines() {
        let (r1, r2) = create_task_ranges_from_pair_info(line);
        let fully_contained =
            range_fully_contains_other(&r1, &r2) || range_fully_contains_other(&r2, &r1);

        println!(
            "r1: {:?}, r2: {:?} | fully contained? {}",
            r1, r2, fully_contained
        );

        if fully_contained {
            total_fully_contained += 1;
        }
    }

    println!("Total fully contained: {}", total_fully_contained);
}
```

Notice how we are checking both `range_fully_contains_other(&r1, &r2)` and `range_fully_contains_other(&r2, &r1)`, to make sure we're not only checking that the first range is fully contained by the second, but also to check that whether the second is fully contained by the first.

</details>

---

## Part Two: At least one shared task

It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (`2-4,6-8` and `2-3,4-5`) don't overlap, while the remaining four pairs (`5-7,7-9`, `2-8,3-7`, `6-6,4-6`, and `2-6,4-8`) do overlap:

- `5-7,7-9` overlaps in a single section, 7.
- `2-8,3-7` overlaps all of the sections 3 through 7.
- `6-6,4-6` overlaps in a single section, 6.
- `2-6,4-8` overlaps in sections 4, 5, and 6.

So, in this example, the number of overlapping assignment pairs is 4.

**In how many assignment pairs do the ranges overlap?**

<details>
<summary><strong>See solution</strong></summary>

To solve this part of the challenge, we can use everything we've created for part 1 and just add a new method to check whether or not at least one item from a range is contained in the other:

```rust
pub fn range_overlaps(r1: &RangeInclusive<i32>, r2: &RangeInclusive<i32>) -> bool {
    let mut overlaps = false;

    for n in r2.clone() {
        if r1.contains(&n) {
            overlaps = true;
            break;
        }
    }

    overlaps
}

#[cfg(test)]
mod tests {
    // -- snip --
    #[test]
    fn should_check_whether_or_not_a_range_overlaps_with_the_other() {
        assert!(range_overlaps(&(5..=7), &(7..=9)));
        assert!(!range_overlaps(&(2..=3), &(4..=7)));
    }
}
```

The logic is similar to what was built for part 1, but now we start the `overlaps` variable with `false` and iterate over `r2`, checking whether or not `r1` contains the item `n`. If it returns true for any given item, we set `overlaps` to `true` and exit the loop.

The updated output at `main` looks like:

```rust
fn main() {
    let contents = fs::read_to_string("sample.txt").expect("Should be able to read the file");

    let mut total_fully_contained = 0;
    let mut total_overlaps = 0;
    for line in contents.lines() {
        let (r1, r2) = create_task_ranges_from_pair_info(line);
        let overlaps = range_overlaps(&r1, &r2) || range_overlaps(&r2, &r1);
        let fully_contained =
            range_fully_contains_other(&r1, &r2) || range_fully_contains_other(&r2, &r1);

        if overlaps {
            total_overlaps += 1;
        }

        if fully_contained {
            total_fully_contained += 1;
        }
    }

    println!(
        "Total fully contained: {} | Total overlaps: {}",
        total_fully_contained, total_overlaps
    );
}
```

That's all!

</details>
