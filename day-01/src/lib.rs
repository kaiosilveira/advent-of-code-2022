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

pub fn find_the_elf_carrying_more_calories(report_data: &Vec<&str>) -> i32 {
    let total_calorie_aggregator = create_sorted_sum_of_calorie_reports(report_data);
    total_calorie_aggregator[0]
}

pub fn find_top_three_elves_carrying_more_calories(report_data: &Vec<&str>) -> i32 {
    let total_calorie_aggregator = create_sorted_sum_of_calorie_reports(report_data);

    let top_n = &total_calorie_aggregator[0..3];
    let total = top_n.iter().fold(0, |a, b| a + b);

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_harness_for_part_one() {
        let report_data = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        assert_eq!(24000, find_the_elf_carrying_more_calories(&report_data));
    }

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
