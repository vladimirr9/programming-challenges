use std::{fs::{self}};

fn main() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let mut index: u32 = 0;
    let mut ranking : Vec<(u32, u32)> = Vec::new();
    for chunk in data.split("\n\n") {
        let number : u32 = chunk.split("\n").fold(0, |acc, e: &str| acc + e.parse::<u32>().unwrap());

        ranking.push((index, number));
        index += 1;
    }
    ranking.sort_by(|a, b| b.1.cmp(&a.1));
    for line in ranking.iter() {
        println!("{}, {}", line.0, line.1);
    }
    println!("{}", ranking[0].1 + ranking[1].1 + ranking[2].1);

}