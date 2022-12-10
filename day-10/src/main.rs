use core::panic;

fn main() {
    let input = include_str!("../input.txt");

    // parsing
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[..] {
                ["noop"] => Instruction::Noop,
                ["addx", num] => Instruction::AddX(num.parse().unwrap()),
                _ => panic!("Unknown instruction."),
            }
        })
        .collect();

    // X-register for each cycle
    let mut register = vec![1];
    for i in instructions {
        let last = *register.last().unwrap();
        match i {
            Instruction::AddX(value) => {
                register.push(last);
                register.push(last + value);
            }
            Instruction::Noop => register.push(last),
        }
    }

    println!("Part 1: {}", part_one(&register));
    println!("Part 2:\n{}", part_two(&register));
}

enum Instruction {
    AddX(i32),
    Noop,
}

fn part_one(register: &Vec<i32>) -> i32 {
    register
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(k, v)| (k + 1) as i32 * v)
        .sum()
}

fn part_two(register: &Vec<i32>) -> String {
    register
        .iter()
        .enumerate()
        .map(|(i, pos)| {
            let column = i as i32 % 40;
            match column - pos {
                -1..=1 => "#",
                _ if column == 39 => "\n",
                _ => " ",
            }
        })
        .collect()
}
