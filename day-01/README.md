# Calorie Counting

Challenge URL: https://adventofcode.com/2022/day/1

Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.

To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

## Part I: calorie Counting

The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).

The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.

For example, suppose the Elves finish writing their items' Calories and end up with the following list:

```
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
```

This list represents the Calories of the food carried by five Elves:

- The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
- The second Elf is carrying one food item with 4000 Calories.
- The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
- The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
- The fifth Elf is carrying one food item with 10000 Calories.

In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).

Find the Elf carrying the most Calories. **How many total Calories is that Elf carrying?**

<details>
<summary><strong>See solution details</strong></summary>

### Test harness

In a test-driven development fashion, we can use the example provided in the text above as a test harness, to make sure that, for a given input, we're returning the correct output. We can make this test pass as fast as possible just by hard-coding its return value, so we're always on the green bar, and as soon we're able to replace the hard-coded values by real code, it means we're finished. The test harness extracted from the example is:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_harness_for_part_one() {
        let report_data = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        assert_eq!(
            24000,
            find_the_elf_carrying_more_calories(report_data)
        );
    }
}
```

### Solution

To solve this problem, we need to isolate the calorie reports for each elf. We know that each line contains a number representing the total calories for a given item, unless it's an empty line, which means that all items being carried by a given elf were reported and we're moving on to the next elf. To capture this behavior, we can do the following:

```rust
fn create_report_groups(calorie_report: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut clone = calorie_report.clone();
    clone.push("");

    let mut groups: Vec<Vec<i32>> = vec![];

    let mut group: Vec<i32> = vec![];
    for entry in clone {
        if entry.is_empty() {
            groups.push(group.clone());
            group = vec![];
        } else {
            group.push(
                entry
                    .parse::<i32>()
                    .expect("Should be able to parse an entry to i32"),
            );
        }
    }

    groups
}
```

Each group in the code above represents a report of an elf, i.e., a list containing the calories of each item being carried by this elf. Notice how we're pushing an empty string to the end of the cloned `calorie_report` vector. That's to keep things consistent and to consider the last item of the list as well.

Now, for each report group generated by the function above, we need to sum all of its calories. We can use the `fold` method for that. It should look like this:

```rust
group.iter().fold(0, |a, b| a + b)
```

With that, we can generate all the report groups and aggregate the sum of their calories into a variable:

```rust
let mut total_calorie_aggregator: Vec<i32> = create_report_groups(&report_data)
        .iter()
        .map(|g| g.iter().fold(0, |a, b| a + b))
        .collect();
```

We can also `.sort` and `.reverse` the data in the aggregator, so we have the highest numbers in the initial positions:

```rust
    total_calorie_aggregator.sort();
    total_calorie_aggregator.reverse();
```

Gluing all the parts together, we have:

```rust
pub fn find_the_elf_carrying_more_calories(report_data: Vec<&str>) -> i32 {
    if report_data.is_empty() {
        return 0;
    }

    let mut total_calorie_aggregator: Vec<i32> = create_report_groups(&report_data)
        .iter()
        .map(|g| g.iter().fold(0, |a, b| a + b))
        .collect();

    total_calorie_aggregator.sort();
    total_calorie_aggregator.reverse();

    total_calorie_aggregator[0]
}
```

That's all for part one! Now, we just need to load the file and call the function at `main`:

```rust
fn main() {
    let input = fs::read_to_string("sample.txt").expect("Should be able to read the input file");
    let report_data = input.lines().collect();

    let total_calories = find_the_elf_carrying_more_calories(&report_data);

    println!(
        "The sum of calories carried by the elf with more calories is: {}",
        total_calories
    );
}
```

</details>

---

## Part II: top three

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. **How many Calories are those Elves carrying in total?**

<details>
<summary><strong>See solution details</strong></summary>

### Test harness

Similar to what we did for part one, the test harness for part two is:

```rust
#[cfg(test)]
mod tests {
    // -- snip
    #[test]
    fn test_harness_for_part_two() {
        let report_data = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        assert_eq!(
            45000,
            find_top_three_elves_carrying_more_calories(&report_data)
        );
    }
}
```

### Solution

We can reuse most of the code we wrote for part one, as the aggregation logic is the same, with the only difference being that, now, we need to get the first three items of the aggregator instead of just one. We also need to sum these items to get the total. We can start by extracting a shared function to generate the aggregator:

```rust
pub fn create_sorted_sum_of_calorie_reports(report_data: &Vec<&str>) -> Vec<i32> {
    if report_data.is_empty() {
        return vec![];
    }

    let mut total_calorie_aggregator: Vec<i32> = create_report_groups(&report_data)
        .iter()
        .map(|g| g.iter().fold(0, |a, b| a + b))
        .collect();

    total_calorie_aggregator.sort();
    total_calorie_aggregator.reverse();

    total_calorie_aggregator
}
```

Then, we need to update `find_the_elf_carrying_more_calories` so it uses the new function:

```rust
pub fn find_the_elf_carrying_more_calories(report_data: &Vec<&str>) -> i32 {
    let total_calorie_aggregator = create_sorted_sum_of_calorie_reports(report_data);
    total_calorie_aggregator[0]
}
```

Now, we just need to implement our new function to take the top three:

```rust
pub fn find_top_three_elves_carrying_more_calories(report_data: &Vec<&str>) -> i32 {
    let total_calorie_aggregator = create_sorted_sum_of_calorie_reports(report_data);

    let top_n = &total_calorie_aggregator[0..3];
    let total = top_n.iter().fold(0, |a, b| a + b);

    total
}
```

And we're done with part two. The updated code for `main` is:

```rust
fn main() {
    let input = fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let report_data = input.lines().collect();

    let total_calories = find_the_elf_carrying_more_calories(&report_data);
    let total_top_three_calories = find_top_three_elves_carrying_more_calories(&report_data);

    println!(
        "The sum of calories carried by the elf with more calories is: {}",
        total_calories
    );
    println!(
        "The sum of calories carried by the top three elves with more calories is: {}",
        total_top_three_calories
    );
}
```

And that's it!

</details>
