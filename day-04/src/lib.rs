use std::ops::RangeInclusive;

pub fn create_range_of_tasks(task_range_info: &str) -> RangeInclusive<i32> {
    let parts: Vec<&str> = task_range_info.split("-").collect();
    let start: i32 = parts.get(0).unwrap().parse().unwrap();
    let end: i32 = parts.get(1).unwrap().parse().unwrap();

    start..=end
}

pub fn create_task_ranges_from_pair_info(str: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let parts: Vec<&str> = str.split(",").collect();

    (
        create_range_of_tasks(parts.get(0).unwrap()),
        create_range_of_tasks(parts.get(1).unwrap()),
    )
}

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
    use super::*;

    #[test]
    fn should_create_a_range_from_a_string() {
        let range = create_range_of_tasks("2-8");
        let range: Vec<i32> = range.collect();

        assert_eq!(&2, range.first().unwrap());
        assert_eq!(&8, range.last().unwrap());
    }

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

    #[test]
    fn should_check_whether_or_not_a_range_fully_contains_the_other() {
        assert!(range_fully_contains_other(&(2..=3), &(2..=2)));
        assert!(range_fully_contains_other(&(2..=8), &(3..=7)));
        assert!(!range_fully_contains_other(&(3..=7), &(2..=8)));
    }

    #[test]
    fn should_check_whether_or_not_a_range_overlaps_with_the_other() {
        assert!(range_overlaps(&(5..=7), &(7..=9)));
        assert!(!range_overlaps(&(2..=3), &(4..=7)));
    }
}
