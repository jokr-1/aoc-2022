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

    // pathfinder2(&graph)
    let res = pathfinder3(&graph);

    let mut best = 0;
    for (a, p1) in &res {
        for (b, p2) in &res {
            if (a & b) == 0 {
                best = best.max(p1 + p2)
            }
        }
    }
    dbg!(best);
    0
}

fn pathfinder(graph: &HashMap<String, Vec<(String, u32, u32)>>) -> u32 {
    let mut queue = Vec::new();
    let mut res = vec![];
    let start = "AA".to_owned();
    let ids: HashMap<String, u8> = graph
        .keys()
        .enumerate()
        .map(|(k, v)| (v.to_owned(), k as u8))
        .collect();
    queue.push((30, 0, start, 0u64));

    while let Some((time, pressure, current, path)) = queue.pop() {
        for (next, distance, rate) in &graph[&current] {
            if !(path & (1 << ids[next]) == 0) || *distance as isize > time as isize - 1 {
                continue;
            }

            queue.push((
                time - distance - 1,
                pressure + rate * (time - distance - 1),
                next.clone(),
                path | (1 << ids[next]),
            ))
        }

        res.push(pressure);
    }
    *res.iter().max().unwrap()
}

fn pathfinder3(graph: &HashMap<String, Vec<(String, u32, u32)>>) -> Vec<(u64, u32)> {
    let mut queue = Vec::new();
    let mut res = vec![];
    let start = "AA".to_owned();
    let ids: HashMap<String, u8> = graph
        .keys()
        .enumerate()
        .map(|(k, v)| (v.to_owned(), k as u8))
        .collect();
    queue.push((26, 0, start, 0u64));

    while let Some((time, pressure, current, path)) = queue.pop() {
        for (next, distance, rate) in &graph[&current] {
            if !(path & (1 << ids[next]) == 0) || *distance as isize > time as isize - 1 {
                continue;
            }

            queue.push((
                time - distance - 1,
                pressure + rate * (time - distance - 1),
                next.clone(),
                path | (1 << ids[next]),
            ))
        }
        if pressure > 0 {
            res.push((path, pressure));
        }
    }
    res
}

fn pathfinder2(graph: &HashMap<String, Vec<(String, u32, u32)>>) -> u32 {
    let mut queue = Vec::new();
    let mut res = vec![];
    let start = "AA".to_owned();
    let ids: HashMap<String, u8> = graph
        .keys()
        .enumerate()
        .map(|(k, v)| (v.to_owned(), k as u8))
        .collect();
    let graph2: Vec<&Vec<(String, u32, u32)>> = graph.values().collect();
    queue.push((26, 26, 0, 0u64, &start, &start, true));

    while let Some((
        time_me,
        time_elephant,
        pressure,
        path,
        current_me,
        current_elephant,
        elephants_round,
    )) = queue.pop()
    {
        let current = if elephants_round {
            current_elephant
        } else {
            current_me
        };

        let time = if elephants_round {
            time_elephant
        } else {
            time_me
        };

        for (next, distance, rate) in &graph[current] {
            if !(path & (1 << ids[next]) == 0) || *distance as isize > time as isize - 2 {
                continue;
            }
            // println!("{path:b}");
            if elephants_round {
                queue.push((
                    time_me,
                    time - distance - 1,
                    pressure + rate * (time - distance - 1),
                    path | (1 << ids[next]),
                    current_me,
                    next,
                    false,
                ));
            } else {
                queue.push((
                    time - distance - 1,
                    time_elephant,
                    pressure + rate * (time - distance - 1),
                    path | (1 << ids[next]),
                    next,
                    current_elephant,
                    true,
                ));
            }
        }

        res.push(pressure);
    }
    *res.iter().max().unwrap()
}

fn part_two(input: &str) -> i32 {
    0
}
