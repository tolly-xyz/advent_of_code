use std::fs;
use std::path::Path;

pub fn read_input(file: &str) -> Option<String> {
    let day = Path::new(file).file_stem()?.to_str()?;
    let path: String = format!("inputs/{}.txt", day);
    fs::read_to_string(path).ok()
}
