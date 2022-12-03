use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let trimmed = input.trim();
    println!("Part 1: {:?}", part_one(trimmed));
    println!("Part 2: {:?}", part_two(trimmed));
}

fn get_value(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 38
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            a.chars()
                .filter(|item| b.contains(*item))
                .map(get_value)
                .next() // only intersted in first result
                .unwrap()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|badges| {
            badges[0]
                .chars()
                .filter(|item| badges[1].contains(*item) && badges[2].contains(*item))
                .map(get_value)
                .next() // only intersted in first result
                .unwrap()
        })
        .sum()
}
