const CHAR_PRIORITY_LOOKUP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_1(contents: &Vec<&str>) -> usize {
    let mut total = 0;

    for line in contents {
        let (first_part, second_part) = split_item_list_into_two_compartments(line);

        let mut shared_item: char = '0';
        for c in first_part.chars() {
            if second_part.contains(c) {
                shared_item = c;
                break;
            };
        }

        let item_priority = get_char_priority(shared_item);

        total += item_priority;
    }

    total
}

pub fn part_2(contents: &Vec<&str>) -> usize {
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
                break;
            }
        }

        let priority = get_char_priority(badge_char);

        total += priority;
    }

    total
}

pub fn get_char_priority(c: char) -> usize {
    CHAR_PRIORITY_LOOKUP
        .find(c)
        .expect("should be able to find the badge char in the lookup table")
        + 1
}

pub fn split_item_list_into_two_compartments(item_list: &str) -> (&str, &str) {
    let length = item_list.len();
    let middle = length / 2;
    let first_part: &str = &item_list[..middle];
    let second_part: &str = &item_list[middle..length];

    (first_part, second_part)
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

            assert_eq!(157, part_1(&contents));
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

            assert_eq!(70, part_2(&contents));
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

    mod split_item_list_into_two_compartments {
        use super::*;

        #[test]
        fn should_split_the_item_list_into_two_compartments() {
            let item_list = "vJrwpWtwJgWrhcsFMMfFFhFp";

            let (first_part, second_part) = split_item_list_into_two_compartments(item_list);

            assert_eq!("vJrwpWtwJgWr", first_part);
            assert_eq!("hcsFMMfFFhFp", second_part);
        }
    }
}
