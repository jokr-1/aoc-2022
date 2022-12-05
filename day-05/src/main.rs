use std::{collections::LinkedList, vec};

struct Command {
    from: usize,
    to: usize,
    count: usize,
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

    fn move_item(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let node = self.data[from].pop_back().unwrap();
            self.data[to].push_back(node);
        }
    }

    fn shift_item(&mut self, from: usize, to: usize, count: usize) {
        let split_index = self.data[from].len() - count;
        let mut split = self.data[from].split_off(split_index);
        self.data[to].append(&mut split);
    }

    fn top(self) -> String {
        self.data.iter().map(|l| l.back().unwrap()).collect()
    }
}

fn main() {
    let input = include_str!("../input");
    let (stack_def, cmds_def) = input.split_once("\n\n").unwrap();

    let cmds: Vec<Command> = cmds_def
        .trim()
        .lines()
        .map(|line| {
            let vals: Vec<usize> = line
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|v| v.parse().unwrap())
                .collect(); // count, from, to
            Command {
                count: vals[0],
                from: vals[1] - 1,
                to: vals[2] - 1,
            }
        })
        .collect();

    // part 1
    let mut stack = Stack::from_str(stack_def);
    for cmd in cmds.iter() {
        stack.move_item(cmd.from, cmd.to, cmd.count)
    }
    println!("Part 1: {}", stack.top());

    // part 2
    let mut stack = Stack::from_str(stack_def);
    for cmd in cmds.iter() {
        stack.shift_item(cmd.from, cmd.to, cmd.count)
    }
    println!("Part 2: {}", stack.top());
}
