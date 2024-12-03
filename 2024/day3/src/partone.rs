pub fn partone() {
    let input = std::fs::read_to_string("./src/input.txt").expect("read_error");

    let mut sum = 0;

    for line in input.lines() {
        sum += parse_mul(line);
    }

    println!("{:?}", sum)
}

fn parse_mul(input: &str) -> i64 {
    let mut results = 0;
    let mut current_pos = 0;

    while let Some(start_idx) = input[current_pos..].find("mul(") {
        let start_pos = current_pos + start_idx;
        let numbers_start = start_pos + 4;

        if let Some(potential_mul) = extract(&input[numbers_start..]) {
            if let Some((a, b)) = parse_number_pair(potential_mul) {
                results += a * b;
            }
        }

        current_pos = start_pos + 4;
    }

    results
}

fn extract(input: &str) -> Option<&str> {
    let end_idx = input.find(')')?;

    let content = &input[..end_idx];

    if content.chars().all(|c| c.is_ascii_digit() || c == ',')
        && content.chars().filter(|&c| c == ',').count() == 1
    {
        Some(content)
    } else {
        None
    }
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
