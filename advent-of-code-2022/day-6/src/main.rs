use std::{collections::HashSet, fs};

fn main() {
    // first_problem();
    second_problem();
}

fn second_problem() {
    let n = 14;
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    for i in 0..data.len() {
        if are_N_different_chained(data, i, n) {
            println!("{}", i + n);
            break;
        }
    }
}

fn first_problem() {
    let n = 4;
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    for i in 0..data.len() {
        if are_N_different_chained(data, i, n) {
            println!("{}", i + n);
            break;
        }
    }
}

fn are_N_different_chained(data: &str, i: usize, n: usize) -> bool {
    let mut set: HashSet<char> = HashSet::new();

    for j in 0..n {
        if !set.insert(data.chars().nth(i + j).unwrap()) {
            return false;
        }
    }
    return true;
}
