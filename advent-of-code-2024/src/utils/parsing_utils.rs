use std::fs;

pub fn get_contents(day: u8, example: bool) -> String {
    let path = if example {format!("./src/bin/day_{day}/example.txt")} else {format!("./src/bin/day_{day}/puzzle.txt")};
    return fs::read_to_string(path).expect("File must exist!");
}
