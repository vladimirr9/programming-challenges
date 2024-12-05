use std::fs;

pub fn get_contents(day: u8, example: bool) -> String {
    let path = if example {format!("./src/bin/day_{day}/input_example.txt")} else {format!("./src/bin/day_1/input_puzzle.txt")};
    return fs::read_to_string(path).expect("File must exist!");
}
