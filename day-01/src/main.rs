use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> String {
    input
        .trim()
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .lines()
                .map(|value| value.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn part_two(input: &str) -> String {
    let calories: BinaryHeap<u32> = input
        .trim()
        .split("\n\n")
        .map(|calories_per_elf| {
            calories_per_elf
                .lines()
                .map(|value| value.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    calories.iter().take(3).sum::<u32>().to_string()
}
