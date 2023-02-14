use std::{collections::HashSet, fs};

fn main() {
    first_problem();
    second_problem();
}

fn second_problem() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let rows: Vec<&str> = data.split("\n").collect();
    let mut total_score: u32 = 0;
    for i in (0..rows.len()).step_by(3) {
        let mut row1: HashSet<char> = rows[i].chars().collect();
        let row2: HashSet<char> = rows[i+1].chars().collect();
        let row3: HashSet<char> = rows[i+2].chars().collect();
        row1.retain(|item| {
            row2.contains(item) && row3.contains(item)
        });
        let char = row1.iter().last().unwrap().clone();
        total_score += get_value_from_char(char)

    }

    println!("{total_score}")
}

fn first_problem() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let rows: Vec<&str> = data.split("\n").collect();
    let mut total_score: u32 = 0;
    for row in rows {
        total_score += get_value_for_row(row)
    }

    println!("{total_score}")
}

fn get_value_for_row(row: &str) -> u32 {
    let (split1, split2) = row.split_at(row.len() / 2);
    let first_set: HashSet<char> = split1.chars().collect();
    let second_set: HashSet<char> = split2.chars().collect();
    let char = first_set.intersection(&second_set).last().unwrap().clone();
    get_value_from_char(char)
}

fn get_value_from_char(char: char) -> u32 {
    if char.is_uppercase() {
        char as u32 - 38
    } else {
        char as u32 - 96
    }
}
