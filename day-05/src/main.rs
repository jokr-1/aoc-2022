use std::{collections::LinkedList, vec};

fn main() {
    let input = include_str!("../input");
    let (stack_def, cmds_def) = input.split_once("\n\n").unwrap();

    let cmds = cmds_def.trim().lines().map(|line| {
        let vals: Vec<usize> = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|v| v.parse().unwrap())
            .collect(); // amount, from, to
        (vals[0], vals[1] - 1, vals[2] - 1)
    });

    let mut stack_a = Stack::from_str(stack_def);
    let mut stack_b = Stack::from_str(stack_def);

    for (amount, from, to) in cmds {
        (0..amount).for_each(|_| stack_a.move_items(1, from, to));
        stack_b.move_items(amount, from, to);
    }

    println!("Part 1: {}", stack_a.top());
    println!("Part 2: {}", stack_b.top());
}

struct Stack {
    data: Vec<LinkedList<char>>,
}

impl Stack {
    fn from_str(input: &str) -> Self {
        let n = (input.lines().next().unwrap().chars().count() - 3) / 4 + 1;
        let mut data = vec![LinkedList::new(); n];
        for line in input.lines().take(8) {
            for (idx, item) in line.chars().skip(1).step_by(4).enumerate() {
                if item.is_alphanumeric() {
                    data[idx].push_front(item);
                }
            }
        }
        Self { data }
    }

    fn move_items(&mut self, amount: usize, from: usize, to: usize) {
        let split_at = self.data[from].len() - amount;
        let mut split = self.data[from].split_off(split_at);
        self.data[to].append(&mut split);
    }

    fn top(self) -> String {
        self.data.iter().map(|l| l.back().unwrap()).collect()
    }
}
