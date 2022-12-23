use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug)]
struct Shape(Vec<Coordinate>);

impl FromIterator<Coordinate> for Shape {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Shape {
    fn offset(&self, x: isize, y: isize) -> Shape {
        self.0
            .iter()
            .map(|c| Coordinate {
                x: c.x + x,
                y: c.y + y,
            })
            .collect()
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn offset(&self, x: isize, y: isize) -> Coordinate {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Coordinate) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

const SHAPEDEF: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

fn part_one(input: &str) -> isize {
    let shapes = SHAPEDEF
        .split("\n\n")
        .map(|p| {
            p.lines()
                .rev()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| match c {
                        '#' => Some(Coordinate {
                            x: x as isize,
                            y: y as isize,
                        }),
                        _ => None,
                    })
                })
                .collect::<Shape>()
        })
        .collect::<Vec<Shape>>();

    let n = shapes.len();
    let mut shape_count = 0;
    let mut fixed: HashSet<Coordinate> = HashSet::default();

    let mut c = shapes[shape_count].offset(2, 3);
    let (mut dx, mut dy) = (0, 0);
    let mut delta: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
    let mut height_gain = Vec::new();
    let mut last_height = 0;

    for (idx, ch) in input.chars().enumerate().cycle() {
        // draw(&fixed, &c);

        // spawn
        let cand = match ch {
            '>' => c.offset(1, 0),
            '<' => c.offset(-1, 0),
            _ => panic!("Unknown move :("),
        };

        if !cand
            .0
            .iter()
            .any(|c| c.x >= 7 || c.x < 0 || fixed.contains(c))
        {
            dx += cand.0[0].x - c.0[0].x;
            c = cand;
        }

        // falling candidate
        let cand = c.offset(0, -1);

        if cand.0.iter().any(|pos| fixed.contains(pos) || pos.y < 0) {
            for pos in &c.0 {
                fixed.insert(*pos);
            }
            shape_count += 1;
            let max_heiht = fixed.iter().map(|c| c.y).max().unwrap();

            dx = 0;
            dy = 0;
            let max_height = fixed.iter().map(|c| c.y).max().unwrap();
            height_gain.push(last_height - max_heiht);
            c = shapes[shape_count % shapes.len()].offset(2, max_heiht + 4);
            last_height = max_height;

            if shape_count == 2022 {
                dbg!(&height_gain);
                return max_height + 1;
            }
        } else {
            c = cand;
        }
    }

    0
}

fn draw(fixed: &HashSet<Coordinate>, cand: &Shape) {
    for y in (0..20).rev() {
        print!("|");
        for x in 0..7 {
            let p = Coordinate { x, y };
            if fixed.contains(&p) {
                print!("#")
            } else if cand.0.contains(&p) {
                print!("@")
            } else {
                print!(".")
            }
        }
        print!("|");
        println!();
    }
    println!("+-------+")
}

fn part_two(input: &str) -> usize {
    let shapes = SHAPEDEF
        .split("\n\n")
        .map(|p| {
            p.lines()
                .rev()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().filter_map(move |(x, c)| match c {
                        '#' => Some(Coordinate {
                            x: x as isize,
                            y: y as isize,
                        }),
                        _ => None,
                    })
                })
                .collect::<Shape>()
        })
        .collect::<Vec<Shape>>();

    let mut shape_count = 0;
    let mut fixed: HashSet<Coordinate> = HashSet::default();

    let mut c = shapes[shape_count].offset(2, 3);
    let mut height_gain = Vec::new();
    let mut last_height = 0;

    for ch in input.chars().cycle() {
        // draw(&fixed, &c);

        // spawn
        let cand = match ch {
            '>' => c.offset(1, 0),
            '<' => c.offset(-1, 0),
            _ => panic!("Unknown move :("),
        };

        if !cand
            .0
            .iter()
            .any(|c| c.x >= 7 || c.x < 0 || fixed.contains(c))
        {
            c = cand;
        }

        // falling candidate
        let cand = c.offset(0, -1);

        if cand.0.iter().all(|pos| !fixed.contains(pos) && pos.y >= 0) {
            c = cand;
            continue;
        }

        for pos in &c.0 {
            fixed.insert(*pos);
        }
        shape_count += 1;
        let max_heiht = fixed.iter().map(|c| c.y).max().unwrap();
        let max_height = fixed.iter().map(|c| c.y).max().unwrap();
        height_gain.push(max_heiht - last_height);
        c = shapes[shape_count % shapes.len()].offset(2, max_heiht + 4);
        last_height = max_height;

        if height_gain.len() == 5001 {
            break;
        }
    }

    let MAX = 1000000000000;

    for x in 0..2000 {
        for s in 20..2000 {
            let c: Vec<&[isize]> = height_gain[x..].chunks(s).take(2).collect();
            let (a, b) = (c[0], c[1]);
            if a == b {
                let n = (MAX - x) / s;
                let m = (MAX - x) % s;
                let e: isize = a.iter().sum();
                let r: isize = a[..m].iter().sum();
                let t: isize = height_gain[..x].iter().sum();

                return n * e as usize + r as usize + t as usize + 1;
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("../input.txt").trim();
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}
