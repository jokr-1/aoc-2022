use std::fs;

fn main() {
    let input = fs::read_to_string("testinput").unwrap();
    println!("Part 1: {}", part_one(input.trim()));
    println!("Part 2: {}", part_two(input.trim()));
}

fn part_one(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            a.chars()
                .filter(|item| b.contains(*item))
                .map(|item| {
                    if item.is_lowercase() {
                        item as u32 - 96
                    } else {
                        item as u32 - 38
                    }
                })
                .next()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn part_two(input: &str) -> String {
    "".to_string()
}
