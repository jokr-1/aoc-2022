use std::{collections::HashMap, ops::Add, str::FromStr};

#[derive(Debug)]
enum Job {
    Number(isize),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

impl FromStr for Job {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let res = match parts[..] {
            [a, "+", b] => Job::Add(a.to_owned(), b.to_owned()),
            [a, "-", b] => Job::Subtract(a.to_owned(), b.to_owned()),
            [a, "*", b] => Job::Multiply(a.to_owned(), b.to_owned()),
            [a, "/", b] => Job::Divide(a.to_owned(), b.to_owned()),
            [value] => Job::Number(value.parse().unwrap()),
            _ => panic!("Unknown row..."),
        };
        Ok(res)
    }
}

fn resolve(monkeys: &HashMap<String, Job>, root: &String) -> isize {
    match &monkeys[root] {
        Job::Number(x) => *x,
        Job::Add(a, b) => resolve(monkeys, &a) + resolve(monkeys, &b),
        Job::Subtract(a, b) => resolve(monkeys, &a) - resolve(monkeys, &b),
        Job::Multiply(a, b) => resolve(monkeys, &a) * resolve(monkeys, &b),
        Job::Divide(a, b) => resolve(monkeys, &a) / resolve(monkeys, &b),
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let monkeys: HashMap<String, Job> = input
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();
            (name.to_owned(), job.parse().unwrap())
        })
        .collect();

    println!("Part 1: {:?}", resolve(&monkeys, &"root".to_owned()));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    0
}

fn part_two(input: &str) -> i32 {
    0
}
