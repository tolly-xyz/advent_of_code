use std::path::Path;

use aoc_2023::read_input;

fn part_one(input: &str) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let first: char = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last: char = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        sum += format!("{first}{last}").parse::<u32>().unwrap();
    }

    println!("Part One Answer: {}", sum)
}

fn part_two(input: &str) {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if first == 0 {
                    first = c.to_digit(10).unwrap();
                }
                last = c.to_digit(10).unwrap();
            } else if let Some((j, _)) = numbers.iter().enumerate().find(|(_, num)| {
                let end = i + num.len();
                end - 1 < line.len() && ***num == line[i..end]
            }) {
                if first == 0 {
                    first = j as u32 + 1;
                }
                last = j as u32 + 1;
            }
        }
        sum += 10 * first + last;
    }

    println!("Part Two Answer: {}", sum)
}

fn main() {
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let input: String = read_input(day);

    part_one(&input);
    part_two(&input);
}
