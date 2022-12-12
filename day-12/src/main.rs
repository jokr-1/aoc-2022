use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

type Position = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Node {
    cost: usize,
    position: Position,
}

// Implementing Ord and PartialOrd to get a Min-Heap for Node
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&other.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, Position, Position) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start = (i, j);
                        'a'
                    }
                    'E' => {
                        end = (i, j);
                        'z'
                    }
                    _ => c,
                })
                .collect()
        })
        .collect();

    (grid, start, end)
}

fn djikstra(
    edges: &BTreeMap<Position, Vec<Position>>,
    start: Position,
    end: Position,
) -> Option<usize> {
    let mut dist: BTreeMap<Position, usize> = edges.keys().map(|pos| (*pos, usize::MAX)).collect();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);

    heap.push(Node {
        cost: 0,
        position: start,
    });

    while let Some(Node { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for edge in &edges[&position] {
            let candidate = Node {
                cost: cost + 1, // +1 costs for any step
                position: *edge,
            };

            if candidate.cost < dist[&candidate.position] {
                heap.push(candidate);
                *dist.get_mut(&candidate.position).unwrap() = candidate.cost;
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("../input.txt");
    let (grid, start, end) = parse_grid(input);

    let (n, m): (usize, usize) = (grid.len(), grid[0].len());
    let positions: Vec<Position> = (0..n).flat_map(|i| (0..m).map(move |j| (i, j))).collect();

    // map from position to valid neighbours
    let edges: BTreeMap<Position, Vec<Position>> = positions
        .iter()
        .map(|&(i, j)| {
            let adjacents = [(-1, 0), (1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|(di, dj)| (di + i as isize, dj + j as isize))
                .filter_map(|(ia, ja)| {
                    if (0..n as isize).contains(&ia) && (0..m as isize).contains(&ja) {
                        Some((ia as usize, ja as usize))
                    } else {
                        None
                    }
                })
                .filter(|&(ia, ja)| grid[ia][ja] as usize <= grid[i][j] as usize + 1)
                .collect();
            ((i, j), adjacents)
        })
        .collect();

    // PART 1
    let solution = djikstra(&edges, start, end).unwrap();
    println!("Part 1: {solution}");

    // PART 2
    let solution = positions
        .iter()
        .filter_map(|&(i, j)| match grid[i][j] {
            'a' => djikstra(&edges, (i, j), end),
            _ => None,
        })
        .min()
        .unwrap();

    println!("Part 2: {solution}");
}
