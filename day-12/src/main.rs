use std::collections::{HashMap, HashSet, VecDeque};

type Position = (usize, usize);

fn bfs<F, G>(
    adjacents: &HashMap<Position, Vec<Position>>,
    start: Position,
    goal: F,
    reachable: G,
) -> Option<usize>
where
    F: Fn(&Position) -> bool,
    G: Fn(&Position, &Position) -> bool,
{
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((node, cost)) = queue.pop_front() {
        if goal(&node) {
            return Some(cost);
        }

        for next in &adjacents[&node] {
            if visited.contains(&next) {
                continue;
            }

            queue.push_back((*next, cost + 1));
            visited.insert(*next);
        }
    }
    None
}

fn main() {
    let input = include_str!("../input.txt");

    let mut start = (0, 0);
    let mut end = (0, 0);

    let graph: HashMap<Position, u8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .map(|(pos, c)| {
            let c = match c {
                'S' => {
                    start = pos;
                    'a'
                }
                'E' => {
                    end = pos;
                    'z'
                }
                _ => c,
            };
            (pos, c as u8)
        })
        .collect();

    let adjacents: HashMap<Position, Vec<Position>> = graph
        .iter()
        .map(|(&pos, &node_val)| {
            let n = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|(di, dj)| {
                    let next = (
                        (pos.0 as isize + di) as usize,
                        (pos.1 as isize + dj) as usize,
                    );
                    if let Some(val) = graph.get(&next) {
                        if *val <= node_val + 1 {
                            Some(next)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Position>>();
            (pos, n)
        })
        .collect();

    let adjacents2: HashMap<Position, Vec<Position>> = graph
        .iter()
        .map(|(&pos, &node_val)| {
            let n = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|(di, dj)| {
                    let next = (
                        (pos.0 as isize + di) as usize,
                        (pos.1 as isize + dj) as usize,
                    );
                    if let Some(val) = graph.get(&next) {
                        if node_val <= *val + 1 {
                            Some(next)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Position>>();
            (pos, n)
        })
        .collect();

    // dbg!(&adjacents[&(0, 0)]);

    for _ in 0..1000 {
        // PART 1
        let solution = bfs(
            &adjacents,
            start,
            |node| node == &end,                         // goal
            |node, next| graph[next] <= graph[node] + 1, // reachable adjacents
        )
        .unwrap();
        // println!("Part 1: {solution}");

        // PART 2
        let solution = bfs(
            &adjacents2,
            end,
            |node| graph[node] == 'a' as u8,             // goal
            |node, next| graph[node] - 1 <= graph[next], // reachable adjacents
        )
        .unwrap();
        // println!("Part 2: {solution}");
    }
}
