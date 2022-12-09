fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    0
}

fn part_two(input: &str) -> i32 {
    0
}

#[test]
fn test() {
    let input = include_str!("../testinput.txt");
    assert_eq!(part_one(input), 1000);
    assert_eq!(part_two(input), 1000);
}
