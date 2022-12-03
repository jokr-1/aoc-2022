use std::fs;

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input")?;
    println!("Part 1: {:?}", part_one(input.trim()));
    println!("Part 2: {:?}", part_two(input.trim()));
    Ok(())
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let me = chars[2] as i32 - 'X' as i32;
            let other = chars[0] as i32 - 'A' as i32;
            match ((me - other) + 3) % 3 {
                1 => 6 + me + 1,
                0 => 3 + me + 1,
                _ => 0 + me + 1,
            }
        })
        .sum()
}

fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let chars: Vec<char> = l.chars().collect();
            let other = chars[0] as i32 - 'A' as i32;
            match chars[2] {
                'X' => 0 + (other + 2) % 3 + 1,
                'Y' => 3 + other + 1,
                _ => 6 + (other + 1) % 3 + 1,
            }
        })
        .sum()
}
