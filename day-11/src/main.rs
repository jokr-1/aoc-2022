#[derive(Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Sqaure,
}

#[derive(Clone)]
struct Monkey {
    operation: Operation,
    div_by: u64,
    receiver: (usize, usize),
    items: Vec<u64>,
}

fn parse_monkey(monkey: &str) -> Monkey {
    let mut lines = monkey.lines().skip(1);

    let items: Vec<u64> = lines
        .next()
        .unwrap()
        .split([':', ','])
        .skip(1)
        .map(|v| v.trim().parse().unwrap())
        .collect();

    let parts: Vec<&str> = lines.next().unwrap().split_whitespace().collect();

    let operation = match parts[..] {
        [.., "*", "old"] => Operation::Sqaure,
        [.., "+", v] => Operation::Add(v.parse().unwrap()),
        [.., "*", v] => Operation::Multiply(v.parse().unwrap()),
        _ => panic!("Unknown operation"),
    };

    let div_by: u64 = lines
        .next()
        .unwrap()
        .split(" by ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let receiver: Vec<usize> = lines
        .map(|l| l.split("monkey ").nth(1).unwrap().parse().unwrap())
        .collect();

    Monkey {
        div_by: div_by,
        operation: operation,
        receiver: (receiver[0], receiver[1]),
        items: items,
    }
}

fn get_monkey_business(mut monkeys: Vec<Monkey>, divider: u64, rounds: usize) -> u64 {
    let mut inspections = vec![0; monkeys.len()];

    // common divisor to keep values low
    let common_div: u64 = monkeys.iter().map(|m| m.div_by).product();

    for _ in 0..rounds {
        for monkey_id in 0..monkeys.len() {
            let items = monkeys[monkey_id].items.clone();
            monkeys[monkey_id].items.clear();

            for worry_level in items {
                let mut new_value = match monkeys[monkey_id].operation {
                    Operation::Add(value) => worry_level + value,
                    Operation::Multiply(value) => worry_level * value,
                    Operation::Sqaure => worry_level * worry_level,
                };

                new_value /= divider;
                new_value %= common_div;

                let move_to = match new_value % monkeys[monkey_id].div_by {
                    0 => monkeys[monkey_id].receiver.0,
                    _ => monkeys[monkey_id].receiver.1,
                };
                inspections[monkey_id] += 1;

                monkeys[move_to].items.push(new_value);
            }
        }
    }
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn main() {
    let input = include_str!("../input.txt");
    let monkeys: Vec<Monkey> = input.trim().split("\n\n").map(parse_monkey).collect();

    println!("Part 1: {}", get_monkey_business(monkeys.clone(), 3, 20));
    println!("Part 2: {}", get_monkey_business(monkeys.clone(), 1, 10000));
}
