use std::{fs, num::ParseIntError};

fn main() -> Result<(), ParseIntError> {
    let input =
        fs::read_to_string("/home/me/personal/aoc/2024/day1/src/input.txt").expect("read_error");

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let seg = line.split_whitespace().collect::<Vec<&str>>();

        if seg.len() == 2 {
            let a: i32 = seg[0].parse()?;
            list_a.push(a);

            let b: i32 = seg[1].parse()?;
            list_b.push(b);
        }
    }

    list_a.sort();
    list_b.sort();

    let mut distance = 0;

    for i in 0..list_a.len() {
        let a = list_a[i];
        let b = list_b[i];

        distance += a.abs_diff(b)
    }

    println!("{:?}", distance);

    Ok(())
}
