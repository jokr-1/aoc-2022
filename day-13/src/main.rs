use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Value(u8),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];
        let mut list = vec![];
        let mut value = None;

        for c in s.bytes() {
            match c {
                b'0'..=b'9' => {
                    value = Some(match value.take() {
                        None => c - b'0',
                        Some(val) => val * 10 + (c - b'0'),
                    });
                }

                b',' => {
                    if let Some(val) = value.take() {
                        list.push(Packet::Value(val));
                    }
                }

                b'[' => {
                    stack.push((std::mem::take(&mut list), value.take()));
                }

                b']' => {
                    if let Some(val) = value.take() {
                        list.push(Packet::Value(val));
                    }

                    let packet = Packet::List(list);
                    (list, value) = stack.pop().unwrap();
                    list.push(packet);
                }

                _ => panic!("unexpected character: {}", c as char),
            }
        }

        Ok(Packet::List(list))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
            (a @ Packet::Value(_), b @ Packet::List(_)) => Packet::List(vec![a.clone()]).cmp(b),
            (a @ Packet::List(_), b @ Packet::Value(_)) => a.cmp(&Packet::List(vec![b.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .flat_map(|line| line.lines())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<Packet>>()
        .chunks(2)
        .zip(1..)
        .filter(|(chunk, _idx)| chunk[0] < chunk[1])
        .map(|(_c, idx)| idx)
        .sum()
}

fn part_two(input: &str) -> usize {
    let div_a: Packet = "[[2]]".parse().unwrap();
    let div_b: Packet = "[[6]]".parse().unwrap();

    let pos = input
        .split("\n\n")
        .flat_map(|line| line.lines())
        .map(|l| l.parse::<Packet>().unwrap())
        .fold((1, 2), |acc, p| {
            (acc.0 + (div_a > p) as usize, acc.1 + (div_b > p) as usize)
        });

    pos.0 * pos.1
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}
