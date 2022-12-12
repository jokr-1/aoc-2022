use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let commands: Vec<Command> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(d, v)| {
            let direction = match d {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("unknown direction"),
            };
            let amount = v.parse::<usize>().unwrap();
            Command { direction, amount }
        })
        .collect();

    let mut rope = Rope::new(2);
    commands.iter().for_each(|cmd| rope.execute(&cmd));
    println!("Part 1: {:?}", rope.tail_positions.len());

    let mut rope = Rope::new(10);
    commands.iter().for_each(|cmd| rope.execute(&cmd));
    println!("Part 2: {:?}", rope.tail_positions.len());
}

type Position = (i32, i32);

struct Command {
    direction: Direction,
    amount: usize,
}

struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Rope {
    fn new(n: usize) -> Self {
        Self {
            knots: vec![(0, 0); n],
            tail_positions: HashSet::from([(0, 0)]),
        }
    }

    fn execute(&mut self, cmd: &Command) {
        for _ in 0..cmd.amount {
            self.move_head(&cmd.direction);
            self.update_tail();
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.knots[0].1 += 1 as i32,
            Direction::Down => self.knots[0].1 -= 1 as i32,
            Direction::Right => self.knots[0].0 += 1 as i32,
            Direction::Left => self.knots[0].0 -= 1 as i32,
        };
    }

    fn update_tail(&mut self) {
        let mut last_knot = self.knots[0];

        for knot in self.knots.iter_mut().skip(1) {
            let (dx, dy) = (last_knot.0 - knot.0, last_knot.1 - knot.1);

            if dx.abs() > 1 || dy.abs() > 1 {
                knot.0 += dx.signum();
                knot.1 += dy.signum();
            }

            last_knot = *knot;
        }

        self.tail_positions.insert(*self.knots.last().unwrap());
    }
}
