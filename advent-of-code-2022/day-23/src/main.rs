use std::{fs, time::Instant};

fn main() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let padding = 5;

    
    

    let elapsed = now.elapsed();
    println!("Elapsed part 2: {:.2?}", elapsed);
}

