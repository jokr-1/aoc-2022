fn main() {
    let input = include_str!("../input.txt").trim();

    println!("Part 1: {:?}", decode(input, 1, 1));
    println!("Part 2: {:?}", decode(input, 811_589_153, 10));
}

fn decode(input: &str, key: isize, rounds: usize) -> isize {
    let numbers: Vec<isize> = input.lines().map(|l| l.parse().unwrap()).collect();

    let n = numbers.len();
    let mut mixed: Vec<_> = numbers
        .iter()
        .cloned()
        .map(|v| v * key)
        .enumerate()
        .collect();

    for _ in 0..rounds {
        for original_index in 0..n {
            let idx_old = mixed.iter().position(|&n| n.0 == original_index).unwrap();
            let shift = mixed[idx_old].1;
            let idx_new = idx_old as isize + shift;
            let idx_new = idx_new.rem_euclid(n as isize - 1) as usize;

            mixed.remove(idx_old);
            mixed.insert(idx_new, (original_index, shift));
        }
    }

    let idx_zero = mixed.iter().position(|v| v.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|s| mixed[(idx_zero + s) % n as usize].1)
        .sum()
}
