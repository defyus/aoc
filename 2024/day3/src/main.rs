mod partone;
fn main() {
    let mut input = std::fs::read_to_string("./src/input.txt").expect("read_error");

    input = input.trim().replace(['\n', '\r', ' '], "");

    partone::partone();

    let sum = parse(&input);

    println!("{:?}", sum);
}

#[derive(Debug, PartialEq)]
struct Mul {
    a: i64,
    b: i64,
}

fn parse(input: &str) -> i64 {
    let mut results = 0;
    let mut current_pos = 0;
    let mut muls_enabled = true;

    while current_pos < input.len() {
        if let Some(new_pos) = check_state_instruction(&input[current_pos..], &mut muls_enabled) {
            current_pos += new_pos;
            continue;
        }

        if muls_enabled {
            if let Some((mul, new_pos)) = parse_next_mul(&input[current_pos..]) {
                results += mul.a * mul.b;
                current_pos += new_pos;
            } else {
                current_pos += 1;
            }
        } else {
            current_pos += 1;
        }
    }

    results
}

fn check_state_instruction(input: &str, muls_enabled: &mut bool) -> Option<usize> {
    if input.starts_with("do()") {
        *muls_enabled = true;
        return Some(4);
    }

    if input.starts_with("don't()") {
        *muls_enabled = false;
        return Some(7);
    }

    None
}

fn parse_next_mul(input: &str) -> Option<(Mul, usize)> {
    if !input.starts_with("mul(") {
        return None;
    }

    let numbers_start = 4;
    let content = input.get(numbers_start..)?;

    let end_idx = content.find(')')?;
    let numbers_str = &content[..end_idx];

    if let Some((a, b)) = parse_number_pair(numbers_str) {
        return Some((Mul { a, b }, numbers_start + end_idx + 1));
    }

    None
}

fn parse_number_pair(input: &str) -> Option<(i64, i64)> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return None;
    }

    if !parts[0].chars().all(|c| c.is_ascii_digit())
        || !parts[1].chars().all(|c| c.is_ascii_digit())
    {
        return None;
    }

    let a = parts[0].parse::<i64>().ok()?;
    let b = parts[1].parse::<i64>().ok()?;

    Some((a, b))
}
