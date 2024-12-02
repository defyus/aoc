use std::{fs, io::Error, ops::Index};
fn main() {
    let _ = run().unwrap();
}

fn run() -> Result<(), Error> {
    let input = fs::read_to_string("/home/me/personal/aoc/2024/day1/src/input.txt")?;

    let mut list_a = Vec::new();
    let mut list_b = Vec::new();

    for line in input.lines() {
        let seg: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if seg.len() == 2 {
            let a: u32 = seg.get(0).unwrap().parse().unwrap();
            list_a.push(a);

            let b: u32 = seg.get(1).unwrap().parse().unwrap();
            list_b.push(b);
        }
    }

    list_a.sort();
    list_b.sort();

    let mut distance = 0;

    for i in 0..list_a.len() {
        let a = list_a.index(i.clone());
        let b = list_b.index(i);

        if a > b {
            distance += a - b;
        } else {
            distance += b - a;
        }
    }

    println!("{:?}", distance);

    Ok(())
}
