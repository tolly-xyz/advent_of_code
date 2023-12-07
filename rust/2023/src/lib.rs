use std::fs;

pub fn read_input(day: &str) -> String {
    let path: String = format!("inputs/{}.txt", day);
    fs::read_to_string(path).unwrap()
}
