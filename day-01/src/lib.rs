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

fn reduce(arr: &Vec<i32>) -> i32 {
    let mut total = 0;

    for item in arr {
        total += item;
    }

    total
}

pub fn find_top_n_elves_carrying_more_calories(report_data: Vec<&str>, take_top_n: usize) -> i32 {
    if report_data.is_empty() {
        return 0;
    }

    let mut total_calorie_aggregator: Vec<i32> = vec![];
    for group in create_report_groups(&report_data) {
        let partial = reduce(&group);
        total_calorie_aggregator.push(partial);
    }

    total_calorie_aggregator.sort();
    total_calorie_aggregator.reverse();

    let top_n = &total_calorie_aggregator[0..take_top_n];
    let total = reduce(&top_n.to_vec());

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_zero_if_calorie_report_is_empty() {
        let report_data = vec![];
        let take_top_n = 1;

        assert_eq!(
            0,
            find_top_n_elves_carrying_more_calories(report_data, take_top_n)
        );
    }

    #[test]
    fn test_returns_the_value_of_the_first_line_if_report_has_only_one_line() {
        let report_data = vec!["1000"];
        let take_top_n = 1;

        assert_eq!(
            1000,
            find_top_n_elves_carrying_more_calories(report_data, take_top_n)
        );
    }

    #[test]
    fn test_returns_the_sum_of_the_lines_for_a_single_elf() {
        let report_data = vec!["1000", "2000"];
        let take_top_n = 1;

        assert_eq!(
            3000,
            find_top_n_elves_carrying_more_calories(report_data, take_top_n)
        );
    }

    #[test]
    fn test_returns_the_elf_carrying_more_calories_if_there_are_multiple_elves() {
        let report_data = vec!["1000", "2000", "", "500", "500"];
        let take_top_n = 1;

        assert_eq!(
            3000,
            find_top_n_elves_carrying_more_calories(report_data, take_top_n)
        );
    }

    #[test]
    fn test_returns_the_sum_of_top_three_elves_carrying_more_calories() {
        let take_top_n = 3;
        let report_data = vec![
            "1000", "2000", "", "50", "50", "", "500", "500", "", "100", "100",
        ];

        assert_eq!(
            4200,
            find_top_n_elves_carrying_more_calories(report_data, take_top_n)
        );
    }
}
