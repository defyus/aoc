use anyhow::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./src/input.txt").expect("read_error");

    let mut safe_reports = 0;
    let mut total_reports = 0;

    for line in input.lines() {
        total_reports += 1;

        let segments: Vec<i32> = line.split_whitespace().map(parse_report).collect();

        let mut incremental = false;

        let mut passed_levels = 0;

        for idx in 0..segments.len() {
            let current = segments[idx];

            if idx == 0 && idx < segments.len() {
                incremental = &current > &segments[idx + 1];
            }

            if idx > 0 {
                let prev = segments[idx - 1];
                let difference = prev.abs_diff(current);

                if &prev > &current && !incremental {
                    continue;
                }

                if &prev < &current && incremental {
                    continue;
                }

                if &prev == &current {
                    continue;
                }

                if difference > 3 {
                    continue;
                }
            }

            passed_levels += 1;
        }

        if passed_levels == segments.len() {
            safe_reports += 1
        }
    }

    println!("Safe Report: {:?} out of {:?}", safe_reports, total_reports);

    Ok(())
}

fn parse_report(val: &str) -> i32 {
    val.parse::<i32>().expect("invalid_number")
}
