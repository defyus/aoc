use anyhow::Error;
use std::fs;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./src/input.txt").expect("read_error");

    for line in input.lines() {
        let _segments = line
            .split_whitespace()
            .map(|val| val.parse().expect("invalid_number"))
            .collect::<Vec<i32>>();
    }

    Ok(())
}
