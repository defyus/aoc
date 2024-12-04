fn main() {
    let example: Vec<Vec<char>> = std::fs::read_to_string("./src/example.txt")
        .expect("read_error")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Example");
    println!("Part 1: {}", count_xmas(&example));
    println!("Part 2: {}", count_x_mas(&example));
    println!();

    let input: Vec<Vec<char>> = std::fs::read_to_string("./src/input.txt")
        .expect("read_error")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Input");
    println!("Part 1: {}", count_xmas(&input));
    println!("Part 2: {}", count_x_mas(&input));
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;
    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // diagonal down-right
        (-1, 1),  // diagonal up-right
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
    ];
    let target = "XMAS";

    for y in 0..height {
        for x in 0..width {
            for &(dy, dx) in &directions {
                if check_word(grid, x, y, dx, dy, target) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if check_x_pattern(grid, x, y) {
                count += 1;
            }
        }
    }
    count
}

fn check_x_pattern(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let diag1 = [(x - 1, y - 1), (x, y), (x + 1, y + 1)]; // Upper-left to lower-right
    let diag2 = [(x + 1, y - 1), (x, y), (x - 1, y + 1)]; // Upper-right to lower-left

    let diag1_chars: Vec<char> = diag1.iter().map(|&(dx, dy)| grid[dy][dx]).collect();
    let diag2_chars: Vec<char> = diag2.iter().map(|&(dx, dy)| grid[dy][dx]).collect();

    (is_mas(&diag1_chars) || is_sam(&diag1_chars)) && (is_mas(&diag2_chars) || is_sam(&diag2_chars))
}

fn is_mas(chars: &[char]) -> bool {
    chars.iter().collect::<String>() == "MAS"
}

fn is_sam(chars: &[char]) -> bool {
    chars.iter().collect::<String>() == "SAM"
}

fn check_word(
    grid: &[Vec<char>],
    start_x: usize,
    start_y: usize,
    dx: i32,
    dy: i32,
    word: &str,
) -> bool {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let mut chars = Vec::new();

    for i in 0..word.len() {
        let x = start_x as i32 + dx * i as i32;
        let y = start_y as i32 + dy * i as i32;

        if x < 0 || x >= width || y < 0 || y >= height {
            return false;
        }

        chars.push(grid[y as usize][x as usize]);
    }

    chars.iter().collect::<String>() == word
}
