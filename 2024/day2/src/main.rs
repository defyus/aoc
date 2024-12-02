use anyhow::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./src/input.txt").expect("read_error");

    let mut part_one_safe_reports = 0;
    let mut part_two_safe_reports = 0;
    let mut total_reports = 0;

    for line in input.lines() {
        total_reports += 1;

        let segments: Vec<i32> = line.split_whitespace().map(parse_report).collect();

        if valid_predicate(&segments) {
            part_two_safe_reports += 1;
            part_one_safe_reports += 1;
            continue;
        }

        if iter(&segments, true) {
            part_two_safe_reports += 1
        }
    }

    assert!(242 == part_one_safe_reports);

    println!(
        "[Part 1] Safe Report: {:?} out of {:?}",
        part_one_safe_reports, total_reports
    );

    assert!(311 == part_two_safe_reports);

    println!(
        "[Part 2] Safe Report: {:?} out of {:?}",
        part_two_safe_reports, total_reports
    );

    Ok(())
}

fn iter(segments: &Vec<i32>, check: bool) -> bool {
    let mut passed_levels = 0;
    for idx in 0..segments.len() {
        if !valid_predicate(segments) {
            if check {
                let mut temp = segments.clone();
                temp.remove(idx);

                if valid_predicate(&temp) {
                    return true;
                }
            };
            continue;
        }
        passed_levels += 1;
    }
    return passed_levels == segments.len();
}

fn valid_predicate(segments: &Vec<i32>) -> bool {
    let predicate_dsc = |a: &i32, b: &i32| a > b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) != 0;
    let predicate_asc = |a: &i32, b: &i32| a < b && a.abs_diff(*b) <= 3 && a.abs_diff(*b) != 0;

    let valid_dsc = segments.is_sorted_by(predicate_dsc);
    let valid_asc = segments.is_sorted_by(predicate_asc);

    valid_dsc || valid_asc
}

fn parse_report(val: &str) -> i32 {
    val.parse::<i32>().expect("invalid_number")
}
