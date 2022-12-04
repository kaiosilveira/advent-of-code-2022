const CHAR_PRIORITY_LOOKUP: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

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

pub fn part_2(contents: &Vec<&str>) -> usize {
    let groups = create_groups_of_three_items(contents);

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

pub fn split_item_list_into_two_compartments(item_list: &str) -> (&str, &str) {
    let length = item_list.len();
    let middle = length / 2;
    let first_part: &str = &item_list[..middle];
    let second_part: &str = &item_list[middle..length];

    (first_part, second_part)
}

pub fn get_char_priority(c: char) -> usize {
    CHAR_PRIORITY_LOOKUP
        .find(c)
        .expect("should be able to find the badge char in the lookup table")
        + 1
}

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

pub fn create_groups_of_three_items<'a>(contents: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
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

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_total_sum_of_shared_item_priorities {
        use super::*;

        #[test]
        fn should_return_the_correct_sum_of_priorities() {
            /*
              The input below is extracted from the example shared in the challenge description.

              157 is a magic number here, as it is the sum of the shared item's priorities described by the challenge for the input below.
            */

            let contents = vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ];

            assert_eq!(157, get_total_sum_of_shared_item_priorities(&contents));
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

    mod find_shared_item_type_between {
        use super::*;

        #[test]
        fn should_return_none_if_no_shared_item_was_found() {
            let first_str = "def";
            let second_str = "abc";

            assert_eq!(None, find_shared_item_type_between(first_str, second_str));
        }

        #[test]
        fn should_find_the_shared_char_between_two_strings() {
            let first_str = "vJrwpWtwJgWr";
            let second_str = "hcsFMMfFFhFp";

            assert_eq!(
                'p',
                find_shared_item_type_between(first_str, second_str).unwrap()
            );
        }
    }

    mod create_groups_of_three_items {
        use super::*;

        #[test]
        fn should_create_groups_of_three_items() {
            let contents = vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "PmmdzqPrVvPwwTWBwg",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            ];

            let groups = create_groups_of_three_items(&contents);
            assert_eq!(2, groups.len());

            let first_group = &groups[0];
            assert_eq!(3, first_group.len());
            assert_eq!(&"vJrwpWtwJgWrhcsFMMfFFhFp", first_group.get(0).unwrap());
            assert_eq!(&"PmmdzqPrVvPwwTWBwg", first_group.get(1).unwrap());
            assert_eq!(
                &"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                first_group.get(2).unwrap()
            );

            let second_group = &groups[1];
            assert_eq!(3, second_group.len());
            assert_eq!(&"ttgJtRGJQctTZtZT", second_group.get(0).unwrap());
            assert_eq!(&"CrZsJsPPZsGzwwsLwLmpwMDw", second_group.get(1).unwrap());
            assert_eq!(
                &"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                second_group.get(2).unwrap()
            );
        }
    }
}
