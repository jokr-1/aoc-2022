use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("Part 1: {:?}", part_one(input.trim()));
    println!("Part 2: {:?}", part_two(input.trim()));
}

#[derive(Clone, Copy)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn is_subrange(self, other: Range) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn is_overlapping(self, other: Range) -> bool {
        self.min >= other.min && self.min <= other.max
            || self.max >= other.min && self.max <= other.max
    }
}

impl From<&str> for Range {
    fn from(input: &str) -> Self {
        let (min, max) = input.split_once('-').unwrap();
        Range {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split(',')
                .map(|x| Range::from(x))
                .next_tuple()
                .unwrap();
            a.is_subrange(b) || b.is_subrange(a)
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (a, b) = line
                .split(',')
                .map(|x| Range::from(x))
                .next_tuple()
                .unwrap();
            a.is_overlapping(b) || b.is_overlapping(a)
        })
        .count()
}
