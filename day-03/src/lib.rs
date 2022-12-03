const CHAR_PRIORITY_LOOKUP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_1(contents: Vec<&str>) -> usize {
    let mut total = 0;
    for line in contents {
        let line_length = line.len();
        let middle = line_length / 2;
        let first_part: &str = &line[..middle];
        let second_part: &str = &line[middle..line_length];
        println!("first_part: {} | second_part: {}", first_part, second_part);

        let mut shared_item: char = '0';
        for c in first_part.chars() {
            if second_part.contains(c) {
                shared_item = c;
            };
        }

        let item_priority = CHAR_PRIORITY_LOOKUP
            .find(shared_item)
            .expect("Should be able to find the char in the alphabet")
            + 1;

        println!("Shared char: {} [{}]", shared_item, item_priority);

        total += item_priority;
    }

    println!("Sum: {}", total);

    total
}

pub fn part_2(contents: Vec<&str>) -> usize {
    let mut groups: Vec<Vec<&str>> = vec![];

    let mut group: Vec<&str> = vec![];
    let mut counter = 0;
    for line in contents {
        group.push(line);
        counter += 1;

        if counter == 3 {
            groups.push(group.clone());
            group = vec![];
            counter = 0;
        }
    }

    let mut total = 0;
    for group in groups {
        let first_item_list = group
            .get(0)
            .expect("should be able to get the first element of the group");

        let second_item_list = group
            .get(1)
            .expect("should be able to get the second element of the group");

        let third_item_list = group
            .get(2)
            .expect("should be able to get the third element of the group");

        let mut badge_char = '0';
        for char in first_item_list.chars() {
            if second_item_list.contains(char) && third_item_list.contains(char) {
                badge_char = char;
            }
        }

        let priority = get_char_priority(badge_char);

        total += priority;
    }

    println!("The sum of the priorities of badge items are {}", total);

    total
}

pub fn get_char_priority(c: char) -> usize {
    CHAR_PRIORITY_LOOKUP
        .find(c)
        .expect("should be able to find the badge char in the lookup table")
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part_01 {
        use super::*;

        #[test]
        fn should_return_the_correct_sum_of_priorities() {
            let contents = vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ];

            assert_eq!(157, part_1(contents));
        }
    }

    mod part_02 {
        use super::*;

        #[test]
        fn should_return_the_correct_sum_of_priorities() {
            let contents = vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ];

            assert_eq!(70, part_2(contents));
        }
    }

    mod get_char_priority {
        use super::*;

        #[test]
        fn should_return_the_correct_priority_for_lowercase_a() {
            assert_eq!(1, get_char_priority('a'));
        }

        #[test]
        fn should_return_the_correct_priority_for_uppercase_a() {
            assert_eq!(27, get_char_priority('A'));
        }
    }
}
