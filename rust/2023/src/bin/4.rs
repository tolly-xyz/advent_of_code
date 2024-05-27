use std::collections::{HashMap, HashSet};

use aoc_2023::read_input;

fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let (winning_nums, nums) = line.split_once(": ")?.1.split_once(" | ")?;
        let winning_nums: HashSet<u32> = winning_nums
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let nums: HashSet<u32> = nums
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let matches = nums.intersection(&winning_nums).count();

        if matches > 0 {
            sum += 2_u32.pow(matches as u32 - 1)
        }
    }
    Some(sum)
}

fn part_two(input: &str) {
    let mut sum = 0;

    let card_map: HashMap<u32, u32> = HashMap::new();

    for (line, card_num) in input.lines().zip(1..) {
        let (winning_nums, nums) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_nums: HashSet<u32> = winning_nums
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let nums: HashSet<u32> = nums
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let matches = nums.intersection(&winning_nums).count();
    }
    println!("Part Two Answer: {sum}")
}

fn main() {
    let Some(input) = read_input(file!()) else {
        panic!("Unable to read input!");
    };

    println!("Part One Answer: {}", part_one(&input).unwrap())
    // part_two(&input);
}
