use std::fs::read_to_string;
use std::path::Path;

pub fn get_contents(s: &str) -> String {
    let path = Path::new(s);

    read_to_string(path).expect("Should have been able to read the file")
}
