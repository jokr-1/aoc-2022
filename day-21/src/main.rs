use std::{collections::HashMap, str::FromStr};

const HUMN_KEY: &str = "humn";
const ROOT_KEY: &str = "root";

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

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

fn part_one(input: &str) -> isize {
    let monkeys: HashMap<String, Job> = input
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();
            (name.to_owned(), job.parse().unwrap())
        })
        .collect();
    resolve(&monkeys, &"root".to_owned())
}

fn resolve2(monkeys: &HashMap<String, Job>, root: &String, humn: isize) -> isize {
    match &monkeys[root] {
        Job::Number(_) if *root == HUMN_KEY.to_string() => humn,
        Job::Number(x) => *x,
        Job::Add(a, b) => resolve2(monkeys, &a, humn) + resolve2(monkeys, &b, humn),
        Job::Subtract(a, b) => resolve2(monkeys, &a, humn) - resolve2(monkeys, &b, humn),
        Job::Multiply(a, b) => resolve2(monkeys, &a, humn) * resolve2(monkeys, &b, humn),
        Job::Divide(a, b) => resolve2(monkeys, &a, humn) / resolve2(monkeys, &b, humn),
    }
}

fn part_two(input: &str) -> isize {
    let root = &ROOT_KEY.to_owned();

    let monkeys: HashMap<String, Job> = input
        .lines()
        .map(|l| {
            let (name, job) = l.split_once(": ").unwrap();
            (name.to_owned(), job.parse().unwrap())
        })
        .collect();

    // unpack left and right value
    let (a, b) = match &monkeys[root] {
        Job::Add(a, b) => (a, b),
        Job::Subtract(a, b) => (a, b),
        Job::Multiply(a, b) => (a, b),
        Job::Divide(a, b) => (a, b),
        _ => panic!("Root should be a operation"),
    };

    // newton
    let fn_delta = |i| resolve2(&monkeys, a, i as isize) - resolve2(&monkeys, b, i as isize);

    let mut number = 1;
    let mut last_number = 0;
    let mut delta_last = fn_delta(last_number);

    loop {
        let delta = fn_delta(number);

        if delta == 0 {
            return number;
        }

        // gradient
        let m = (delta - delta_last) / (last_number - number);
        let step = delta / m;

        delta_last = delta;
        last_number = number;
        number += step;
    }
}
