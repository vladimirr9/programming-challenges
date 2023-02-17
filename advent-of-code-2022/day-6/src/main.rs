use std::{collections::HashSet, fs};

fn main() {
    first_problem();
}

fn first_problem() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    for i in 0..data.len() {
        if are_four_different_chained(data, i) {
            println!("{}", i + 4);
            break;
        }
    }
}

fn are_four_different_chained(data: &str, i: usize) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for j in 0..4 {
        if !set.insert(data.chars().nth(i + j).unwrap()) {
            return false;
        }
    }
    return true;
}
