use std::path::Path;

use aoc_2023::read_input;

fn part_one(input: &str) {
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    let mut sum = 0;

    for (line, game_num) in input.lines().zip(1u32..) {
        let (_, game_str) = line.split_once(": ").unwrap();
        if game_str.split("; ").all(|round| {
            round.split(", ").all(|s| {
                let (num, color) = s.split_once(' ').unwrap();
                let count = num.parse::<u32>().unwrap();
                match color {
                    "red" => count <= max_red,
                    "green" => count <= max_green,
                    "blue" => count <= max_blue,
                    _ => panic!("Unknown color: {color}"),
                }
            })
        }) {
            sum += game_num
        }
    }
    println!("Part One Answer: {sum}")
}

fn part_two(input: &str) {
    let mut sum = 0;

    for line in input.lines() {
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        let (_, game_str) = line.split_once(": ").unwrap();
        game_str.split("; ").for_each(|round| {
            round.split(", ").for_each(|s| {
                let (num, color) = s.split_once(' ').unwrap();
                let count = num.parse::<u32>().unwrap();
                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => panic!("Unknown color: {color}"),
                }
            })
        });
        sum += min_red * min_green * min_blue;
    }
    println!("Part Two Answer: {sum}")
}

fn main() {
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input: String = read_input(day);

    part_one(&input);
    part_two(&input);
}
