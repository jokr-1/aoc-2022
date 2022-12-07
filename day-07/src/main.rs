use std::collections::HashMap;

type Filesystem = HashMap<String, usize>;

fn main() {
    let input = include_str!("../input").trim();
    let filesystem = parse_filesystem(input);

    println!("Part 1: {:?}", part_one(&filesystem));
    println!("Part 2: {:?}", part_two(&filesystem));
}

fn parse_filesystem(input: &str) -> Filesystem {
    let mut current_directory = Vec::new();
    let mut directories = HashMap::new();

    input
        .split("$ ")
        .skip(1)
        .map(|part| {
            let (expr, body) = part.split_at(2);
            (expr, body.trim())
        })
        .for_each(|cmd| {
            match cmd {
                ("cd", root @ "/") => {
                    current_directory.clear();
                    current_directory.push(root.to_string());
                }
                ("cd", "..") => {
                    current_directory.pop();
                }
                ("cd", dirname) => {
                    let last_dir = current_directory.last().unwrap();
                    current_directory.push(format!("{last_dir}{dirname}/"));
                }
                ("ls", body) => {
                    body.lines()
                        .map(|l| l.split_whitespace().next().unwrap())
                        .filter_map(|filesize| filesize.parse::<usize>().ok())
                        .for_each(|filesize| {
                            current_directory.iter().cloned().for_each(|dir| {
                                directories
                                    .entry(dir)
                                    .and_modify(|size| *size += filesize)
                                    .or_insert(filesize);
                            })
                        });
                }
                _ => panic!("Unknown command"),
            };
        });

    directories
}

fn part_one(filesystem: &Filesystem) -> usize {
    filesystem.values().filter(|&sum| *sum <= 100_000).sum()
}

fn part_two(filesystem: &Filesystem) -> usize {
    const TOTALSIZE: i32 = 70_000_000;
    const FREESPACE: i32 = 30_000_000;

    let used_space = filesystem["/"] as i32;
    let needed = (FREESPACE - TOTALSIZE + used_space) as usize;

    *filesystem
        .values()
        .min_by_key(|f| f.abs_diff(needed))
        .unwrap()
}
