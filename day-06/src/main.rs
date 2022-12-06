use itertools::Itertools;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {:?}", find_start(input, 4));
    println!("Part 2: {:?}", find_start(input, 14));
}

fn find_start(input: &str, windowsize: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(windowsize)
        .position(|window| window.iter().all_unique())
        .unwrap()
        + windowsize
}
