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
    let mut sum: u32 = 0;

    for line in input.lines() {
        let first: char = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last: char = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        sum += format!("{first}{last}").parse::<u32>().unwrap();
    }

    println!("Part One Answer: {}", sum)
}

fn main() {
    let input: String = read_input(1);

    part_one(&input);
    part_two(&input);
}
