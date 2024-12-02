use std::{fs, num::ParseIntError, ops::Mul};

fn main() -> Result<(), ParseIntError> {
    let input =
        fs::read_to_string("/home/me/personal/aoc/2024/day1/src/input.txt").expect("read_error");

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let seg = line.split_whitespace().collect::<Vec<&str>>();
        if seg.len() == 2 {
            list_a.push(seg[0].parse()?);
            list_b.push(seg[1].parse()?);
        }
    }

    list_a.sort();
    list_b.sort();

    let mut distance = 0;
    let mut similarity = 0;

    for i in 0..list_a.len() {
        let a: i32 = list_a[i];

        let mut similar = 0;

        for item in list_b.iter() {
            if a == *item {
                similar += 1;
                println!("{:?} - {:?}", a, item);
            }
        }

        similarity += a.mul(similar);

        let b: i32 = list_b[i];

        distance += a.abs_diff(b)
    }

    assert!(2904518 == distance);

    println!("Distance: {}", distance);

    assert!(18650129 == similarity);

    println!("Similarity Score: {}", similarity);

    Ok(())
}
