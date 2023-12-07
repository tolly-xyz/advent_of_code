use std::fs;

pub fn read_input(day: usize) -> String {
    let path: String = format!("inputs/{day}.txt");
    fs::read_to_string(path).unwrap()
}
