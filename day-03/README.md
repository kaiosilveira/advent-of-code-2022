# Rucksack Reorganization

Challenge URL: https://adventofcode.com/2022/day/3

One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

## Part one: reorganizing items

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

```
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
```

- The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which means its first compartment contains the items `vJrwpWtwJgWr`, while the second compartment contains the items `hcsFMMfFFhFp`. The only item type that appears in both compartments is lowercase p.
- The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`. The only item type that appears in both compartments is uppercase L.
- The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
- The fourth rucksack's compartments only share item type v.
- The fifth rucksack's compartments only share item type t.
- The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. **What is the sum of the priorities of those item types?**

<details>
<summary><strong>See solution</strong></summary>

The first thing we need to do to solve this problem is to split the list of item types into two compartments, to match what's described above. As the list of item types is described in a single string, we can do this split by following these steps:

- Finding the length of the string:

```rust
let length = item_list.len();
```

- Finding the index that represents the middle of the string:

```rust
let middle = length / 2;
```

- Isolating the first part and the second part:

```rust
let first_part: &str = &item_list[..middle];
let second_part: &str = &item_list[middle..length];
```

_Note: We are using ranges to split the string into two parts, where `item_list[..middle]` means "take everything from the start of the string up to the value of `middle`, and `item_list[middle..length]` means "take everything from the middle of the string up to its end`_

The resulting function for splitting the item type list into two compartments looks like:

```rust
pub fn split_item_list_into_two_compartments(item_list: &str) -> (&str, &str) {
    let length = item_list.len();
    let middle = length / 2;
    let first_part: &str = &item_list[..middle];
    let second_part: &str = &item_list[middle..length];

    (first_part, second_part)
}
```

Next up, we need to find the shared item type between the two compartments. As now we have two strings, each of them representing a list of item types contained in a compartment. We can simply iterate over the first list and check whether or not the second list contains a given item. As soon as we find the item, we can exit the loop and say that we have found it! We need to be careful, though: what will the code return if we don't find any match? Thankfully, Rust has the `Option<T>` construct, which allows us to return either `Some(value)` or `None`, and let the calling code decide what to do with that (we'll be back to the caller implementation and revisit this case later). The code looks like this:

```rust
pub fn find_shared_item_type_between(
    first_compartment: &str,
    second_compartment: &str,
) -> Option<char> {
    let mut result: Option<char> = None;

    for item_type in first_compartment.chars() {
        if second_compartment.contains(item_type) {
            result = Some(item_type);
        };
    }

    result
}
```

Finally, the next part is to calculate the priority of the shared item type. As the challenge describes, each character has the priority of its corresponding position in the alphabet, starting with the lowercase chars, followed by the uppercase chars. We will use a `const` string to represent this rule:

```rust
const CHAR_PRIORITY_LOOKUP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
```

Then, we can implement a function to find an item's index inside the lookup string, and add a `+1` to it, as indexes are zero-based:

```rust
pub fn get_char_priority(c: char) -> usize {
    CHAR_PRIORITY_LOOKUP
        .find(c)
        .expect("should be able to find the badge char in the lookup table")
        + 1
}
```

_Note: The `.find` method returns an `Option<T>`, containing the index of the char in the string in case it exists, and `None` otherwise, similar to what we've discussed above. We're using the `.expect` here to cause the program to panic in case we don't find the expected index for a char._

That's all we need! We can glue all these parts together with a function:

```rust
pub fn get_total_sum_of_shared_item_priorities(contents: &Vec<&str>) -> usize {
    let mut total = 0;

    for line in contents {
        let (first_compartment, second_compartment) = split_item_list_into_two_compartments(line);
        let shared_item_type = find_shared_item_type_between(first_compartment, second_compartment)
            .expect("Failed to find a shared item type");

        let item_priority = get_char_priority(shared_item_type);

        total += item_priority;
    }

    total
}
```

_Note: notice how we are using a `.expect` when calling `find_shared_item_type_between`, that's for the exact same reason as we did it for the `.find` method above: causing a panic if we don't find what we expect_

The `contents` argument represents a vector of lists of item types, so it should look like this:

```rust
let contents = vec![
  "vJrwpWtwJgWrhcsFMMfFFhFp",
  "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
  "PmmdzqPrVvPwwTWBwg"
];
```

Now we just need to implement the boilerplate code to load the file at `main` and feed the data into our function:

```rust
fn main() {
    let contents = fs::read_to_string("sample.txt").expect("should be able to read the file");
    let contents: Vec<&str> = contents.lines().collect();
    println!("Part 1: {}", get_total_sum_of_shared_item_priorities(&contents)); // Part 1: 157
}
```

And that's it for part 1!

</details>

---

## Part two: grouping by badge type

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

```
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
```

And the second group's rucksacks are the next three lines:

```
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
```

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. **What is the sum of the priorities of those item types?**

<details>
<summary><strong>See solution</strong></summary>
</details>

