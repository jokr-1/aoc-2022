use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../testinput.txt");
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

fn dfs(valves: &HashMap<String, Valve>) -> u32 {
    let start = "AA";
    let mut queue = Vec::new();
    // let mut visited = HashSet::new();
    let t_max = 30;

    let valve_ids: HashMap<String, u8> = valves
        .keys()
        .enumerate()
        .map(|(idx, key)| (key.to_owned(), idx as u8))
        .collect();

    // queue: (valve_id, accumulated pressure, accumulated_time, set of openend valves)
    queue.push((start.to_string(), 0, 0, 0u64));
    let mut best = 0;

    while let Some((valve, pressure, time, opened)) = queue.pop() {
        if time >= 29 && pressure > 0 {
            // dbg!(pressure);
            best = pressure.max(best);
            continue;
        }

        for next in &valves[&valve].children {
            let next_valve = &valves[next];

            // also take path with openeing the current valve if it wasnt openend and if flowrate is larger than 0
            if opened & (1 << valve_ids[next]) == 0 && next_valve.flow_rate > 0 && time <= 28 {
                let acc_pressure = pressure + (t_max - time - 2) * next_valve.flow_rate;
                queue.push((
                    next.clone(),
                    acc_pressure,
                    time + 2,
                    opened | (1 << valve_ids[next]), // set bit for valve to 1
                ))
            } else if time <= 29 {
                queue.push((next.clone(), pressure, time + 1, opened))
            }
        }
    }

    return best;
}

fn find_best(
    valves: &HashMap<String, Valve>,
    frozen: &HashSet<String>,
    remaining: u32,
    from: String,
) -> (String, u32, u32) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut res = Vec::new();
    queue.push_back((from, remaining));

    while let Some((valve, time)) = queue.pop_front() {
        if time <= 1 {
            continue;
        }

        if visited.contains(&valve) {
            continue;
        }

        visited.insert(valve.clone());

        for next in &valves[&valve].children {
            let potential = if frozen.contains(next) {
                0
            } else {
                &valves[next].flow_rate * (time - 3)
            };
            res.push((potential, next.clone(), time - 3));
            queue.push_back((next.clone(), time - 2));
        }
    }
    res.sort();
    res.reverse();
    (res[0].1.clone(), res[0].0, res[0].2)
}

fn part_one(input: &str) -> i32 {
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

    let res = pathfinder2(&graph, &"AA".to_owned());
    // let res = dfs(&valves);
    dbg!(res.iter().max());
    0
}

fn pathfinder2(
    graph: &HashMap<String, Vec<(String, u32, u32)>>,
    start: &String,
) -> Vec<(u32, u32)> {
    let mut queue = Vec::new();
    let mut res = Vec::new();
    queue.push((0, start, 0, 0, 0));

    while let Some((pressure, node, rate, time, depth)) = queue.pop() {
        if time == 30 {
            res.push((pressure, time));
            return res;
        }

        for (next, distance, flow_rate) in &graph[node] {
            if time + distance + 1 <= 30 {
                let pressure_gain = pressure + flow_rate * (30 - (time + distance + 1));
                // println!("DEPTH {depth} | TIME: {time}: {next}, {distance}, {pressure_gain}");
                queue.push((
                    pressure + flow_rate * (30 - (time + distance + 1)),
                    next,
                    rate + flow_rate,
                    time + distance + 1,
                    depth + 1,
                ))
            }
        }
        queue.sort();
        // queue.reverse();
        dbg!(&queue);
    }

    res
}

fn pathfinder(graph: &HashMap<String, Vec<(String, u32, u32)>>, start: &String) -> u32 {
    let mut queue = Vec::new();
    // let mut visited = HashSet::new();
    let valve_ids: HashMap<String, u8> = graph
        .keys()
        .enumerate()
        .map(|(idx, k)| (k.to_owned(), idx as u8))
        .collect();

    let mut res = 0;
    queue.push((start, 0, 0, 0u64));

    while let Some((node, pressure, time, opened)) = queue.pop() {
        if time <= 30 {
            res = pressure.max(res);
            continue;
        }

        for (next, distance, flow_rate) in &graph[node] {
            if opened & (1 << valve_ids[next]) == 0 && time + distance + 1 <= 30 {
                queue.push((
                    next,
                    pressure + flow_rate * (30 - time - distance - 1),
                    time + distance + 1,
                    opened | (1 << valve_ids[next]),
                ))
            } else if time + distance <= 30 {
                queue.push((next, pressure, time + distance, opened))
            }
        }
    }

    res
}

fn part_two(input: &str) -> i32 {
    0
}
