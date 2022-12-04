use std::fs;

use day_04::{create_ranges_from_pair_info, range_fully_contains_other, range_overlaps};

fn main() {
    let contents = fs::read_to_string("sample.txt").expect("Should be able to read the file");

    let mut total_fully_contained = 0;
    let mut total_overlaps = 0;
    for line in contents.lines() {
        let (r1, r2) = create_ranges_from_pair_info(line);
        let overlaps = range_overlaps(&r1, &r2) || range_overlaps(&r2, &r1);
        let fully_contained =
            range_fully_contains_other(&r1, &r2) || range_fully_contains_other(&r2, &r1);

        println!(
            "r1: {:?}, r2: {:?} | fully contained? {} | overlaps? {}",
            r1, r2, fully_contained, overlaps
        );

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
