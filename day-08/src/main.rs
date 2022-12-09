fn main() {
    let input: &str = include_str!("../testinput.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let forrest: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let (n, m) = (forrest.len(), forrest[0].len());
    let mut visible = vec![0; m * n];

    // left -> right
    for i in 0..n {
        let mut max = -1;
        for j in 0..m {
            let height = forrest[i][j];
            if height <= max {
                continue;
            }
            visible[i * m + j] = 1;
            max = height;
        }
    }

    // right -> left
    for i in 0..n {
        let mut max = -1;
        for j in (0..m).rev() {
            let height = forrest[i][j];
            if height <= max {
                continue;
            }
            visible[i * m + j] = 1;
            max = height;
        }
    }

    // top -> down
    for j in 0..m {
        let mut max = -1;
        for i in 0..n {
            let height = forrest[i][j];
            if height <= max {
                continue;
            }
            visible[i * m + j] = 1;
            max = height;
        }
    }

    // down -> top
    for j in 0..m {
        let mut max = -1;
        for i in (0..n).rev() {
            let height = forrest[i][j];
            if height <= max {
                continue;
            }
            visible[i * m + j] = 1;
            max = height;
        }
    }

    visible.iter().sum()
}

fn part_two(input: &str) -> i32 {
    let forrest: Vec<Vec<i8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let (n, m) = (forrest.len(), forrest[0].len());
    let mut score = vec![0; m * n];

    // left -> right
    for i in 0..n {
        for j in 0..m {
            let height = forrest[i][j];

            // down
            let mut a = 0;
            for k in (i + 1)..n {
                a += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            // up
            let mut b = 0;
            for k in (0..i).rev() {
                b += 1;
                if forrest[k][j] >= height {
                    break;
                }
            }

            // right
            let mut c = 0;
            for k in (j + 1)..m {
                c += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }

            // left
            let mut d = 0;
            for k in (0..j).rev() {
                d += 1;
                if forrest[i][k] >= height {
                    break;
                }
            }
            println!("{} {} {} {} ({i},{j} | {height})", a, b, c, d);
            println!("--------");

            score[i * m + j] = a * b * c * d
        }
    }

    *score.iter().max().unwrap()
}
