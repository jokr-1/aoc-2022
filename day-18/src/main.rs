use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

const DIRS: [(i8, i8, i8); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

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

    droplets
        .iter()
        .flat_map(|(x, y, z)| {
            DIRS.iter()
                .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
        })
        .filter(|pos| !droplets.contains(pos))
        .count()
}

fn part_two(input: &str) -> i32 {
    const SIZE: usize = 40;

    let mut grid = vec![vec![vec![false; SIZE]; SIZE]; SIZE];
    let mut visited = grid.clone();

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
            for (dx, dy, dz) in DIRS {
                queue.push((
                    (x as i8 + dx) as usize,
                    (y as i8 + dy) as usize,
                    (z as i8 + dz) as usize,
                ));
            }
        }
    }

    surface
}
