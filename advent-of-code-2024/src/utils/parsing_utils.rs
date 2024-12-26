use std::fs;

pub fn get_contents(day: u8, example: bool, part: u8) -> String {
    let path = if example {format!("./src/bin/day_{day}/example_{part}.txt")} else {format!("./src/bin/day_{day}/puzzle_{part}.txt")};
    return fs::read_to_string(path).expect("File must exist!");
}
