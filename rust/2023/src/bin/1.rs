use aoc_2023::read_input;

fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let first: char = line.chars().find(|c| c.is_ascii_digit())?;
        let last: char = line.chars().rfind(|c| c.is_ascii_digit())?;
        sum += 10 * first.to_digit(10)? + last.to_digit(10)?
    }

    Some(sum)
}

fn part_two(input: &str) -> Option<u32> {
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
                    first = c.to_digit(10)?
                }
                last = c.to_digit(10)?
            } else if let Some((_, j)) = numbers
                .iter()
                .zip(1..)
                .find(|(num, _)| line[i..].starts_with(*num))
            {
                if first == 0 {
                    first = j;
                }
                last = j;
            }
        }
        sum += 10 * first + last;
    }

    Some(sum)
}

fn main() {
    let Some(input) = read_input(file!()) else {
        panic!("Unable to read input!");
    };

    match part_one(&input) {
        Some(ans) => println!("Part One: {}", ans),
        None => println!("Part one returned None!"),
    }

    match part_two(&input) {
        Some(ans) => println!("Part Two: {}", ans),
        None => println!("Part one returned None!"),
    }
}
