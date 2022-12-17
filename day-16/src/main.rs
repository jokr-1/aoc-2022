use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    children: Vec<String>,
}

// procedure DFS_iterative(G, v) is
//     let S be a stack
//     S.push(v)
//     while S is not empty do
//         v = S.pop()
//         if v is not labeled as discovered then
//             label v as discovered
//             for all edges from v to w in G.adjacentEdges(v) do
//                 S.push(w)

fn bfs(valves: &HashMap<String, Valve>, start: &String, end: &String) -> Option<u32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));

    while let Some((node, distance)) = queue.pop_front() {
        if node == end {
            return Some(distance);
        }

        if visited.contains(node) {
            continue;
        }

        visited.insert(node.clone());

        for next in &valves[node].children {
            queue.push_back((next, distance + 1))
        }
    }

    None
}

fn part_one(input: &str) -> u32 {
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let parts = line.split([';', '=']).collect::<Vec<&str>>();
            let id = parts[0][6..8].to_owned();
            let flow_rate: u32 = parts[1].parse().unwrap();
            let children: Vec<String> = parts[2][23..]
                .trim()
                .split(", ")
                .map(|s| s.to_owned())
                .collect();
            (
                id.clone(),
                Valve {
                    flow_rate,
                    children,
                },
            )
        })
        .collect();

    let significant_valves: Vec<String> = valves
        .iter()
        .filter_map(|(k, v)| {
            if v.flow_rate > 0 || k == &"AA".to_string() {
                Some(k.to_owned())
            } else {
                None
            }
        })
        .collect();

    // let mut connections = HashMap::<(String, String), u32>::new();
    let graph: HashMap<String, Vec<(String, u32, u32)>> = significant_valves
        .iter()
        .map(|a| {
            let connections: Vec<(String, u32, u32)> = significant_valves
                .iter()
                .filter(|b| a != *b)
                .map(|b| {
                    (
                        b.to_owned(),
                        bfs(&valves, a, b).unwrap(),
                        *&valves[b].flow_rate,
                    )
                })
                .collect();
            (a.to_owned(), connections)
        })
        .collect();

    pathfinder(&graph)
}

fn pathfinder(graph: &HashMap<String, Vec<(String, u32, u32)>>) -> u32 {
    let mut queue = Vec::new();
    let mut res = vec![];
    let start = "AA".to_owned();
    queue.push((30, 0, vec![&start]));

    while let Some((time, pressure, path)) = queue.pop() {
        let current = path.last().unwrap();

        for (next, distance, rate) in &graph[*current] {
            if path.contains(&next) || *distance as isize > time as isize - 1 {
                continue;
            }

            let mut p = path.clone();
            p.push(&next);
            queue.push((
                time - distance - 1,
                pressure + rate * (time - distance - 1),
                p,
            ))
        }

        res.push(pressure);
    }
    *res.iter().max().unwrap()
}

fn part_two(input: &str) -> i32 {
    0
}
