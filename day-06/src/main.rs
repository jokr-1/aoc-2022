use itertools::Itertools;

fn main() {
    let input = include_str!("../input").trim();
    println!("Part 1: {:?}", find(input, 4));
    println!("Part 2: {:?}", find(input, 14));
}

fn find(input: &str, windowsize: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(windowsize)
        .enumerate()
        .find(|(_, w)| w.iter().unique().count() == windowsize)
        .unwrap()
        .0
        + windowsize
}
