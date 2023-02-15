use std::{fs};

fn main() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let rows: Vec<&str> = data.split("\n").collect();
    let mut total_sum : u32 = 0;
    for row in rows {
        if is_there_full_overlap_in_row(row) {
            total_sum += 1;
        }
    }
    println!("{total_sum}");
}


fn is_there_full_overlap_in_row(row: &str) -> bool {
    let (section1, section2) = row.split_once(",").expect("Should be split into two at ,");
    let (a,b) = section1.split_once("-").expect("Should be split into two at -");
    let (a, b) : (u32, u32) = (a.parse().unwrap(), b.parse().unwrap());
    let (x,y) = section2.split_once("-").expect("Should be split into two at -");
    let (x, y) : (u32, u32) = (x.parse().unwrap(), y.parse().unwrap());
    (a..b+1).all(|num| (x..y+1).contains(&num)) || (x..y+1).all(|num| (a..b+1).contains(&num))
}
