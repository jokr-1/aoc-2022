use std::{collections::HashSet, ops::Add, str::FromStr};

struct Position {
    x: i8,
    y: i8,
    z: i8,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> usize {
    let droplets: HashSet<(i8, i8, i8)> = input
        .lines()
        .map(|l| {
            let mut iter = l.split(',').map(|v| v.parse::<i8>().unwrap());
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        })
        .collect();
    let mut surface = 0;
    for (x, y, z) in &droplets {
        if !droplets.contains(&(x + 1, *y, *z)) {
            surface += 1;
        }

        if !droplets.contains(&(x - 1, *y, *z)) {
            surface += 1;
        }

        if !droplets.contains(&(*x, y + 1, *z)) {
            surface += 1;
        }

        if !droplets.contains(&(*x, y - 1, *z)) {
            surface += 1;
        }

        if !droplets.contains(&(*x, *y, z + 1)) {
            surface += 1;
        }

        if !droplets.contains(&(*x, *y, z - 1)) {
            surface += 1;
        }
    }

    surface
}

type Grid = Vec<Vec<Vec<bool>>>;

fn part_two(input: &str) -> i32 {
    const SIZE: usize = 40;
    let mut grid: Grid = vec![vec![vec![false; SIZE]; SIZE]; SIZE];
    let mut visited: Grid = vec![vec![vec![false; SIZE]; SIZE]; SIZE];

    input.lines().for_each(|l| {
        let mut iter = l.split(',').map(|v| v.parse::<usize>().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        grid[x + 1][y + 1][z + 1] = true;
    });

    let mut surface = 0;
    let mut queue = vec![(0, 0, 0)];
    while let Some((x, y, z)) = queue.pop() {
        if x >= SIZE || y >= SIZE || z >= SIZE {
            continue;
        }

        if visited[x][y][z] {
            continue;
        }

        if grid[x][y][z] {
            surface += 1;
        } else {
            visited[x][y][z] = true;
            queue.push((x + 1, y, z));
            queue.push(((x as isize - 1) as usize, y, z));
            queue.push((x, y + 1, z));
            queue.push((x, (y as isize - 1) as usize, z));
            queue.push((x, y, z + 1));
            queue.push((x, y, (z as isize - 1) as usize));
        }
    }

    surface
}
