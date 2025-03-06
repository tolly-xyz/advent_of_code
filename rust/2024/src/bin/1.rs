use aoc_2024::read_input;

fn part_one(input: &str) -> u32 {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(parse)
        .unzip();

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(n1, n2)| n1.abs_diff(n2)).sum()
}

fn parse((num1, num2): (&str, &str)) -> (u32, u32) {
    (num1.parse().unwrap(), num2.parse().unwrap())
}

fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = read_input(file!()).expect("Unable to read input!");

    println!("{}", part_one(&input));

    match part_two(&input) {
        Some(ans) => println!("Part Two: {}", ans),
        None => println!("Part one returned None!"),
    }
}
