use std::{collections::HashMap, fs};

fn main() {
    let example = false;
    let path = if example {"./src/bin/day_1/input_example.txt"} else {"./src/bin/day_1/input_puzzle.txt"};
    let contents = fs::read_to_string(path).expect("File must exist!");
    let lines : Vec<&str> = contents.trim().split("\n").collect();
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();

    for line in lines {
        let mut vals = line.split_whitespace().take(2).into_iter();
        let (first_parsed, second_parsed) = (vals.next().unwrap(), vals.next().unwrap());
        first.push(first_parsed.parse().unwrap());
        second.push(second_parsed.parse().unwrap());
    }
    first.sort();
    second.sort();
    let mut sum = 0;
    first.iter().zip(second.iter()).for_each(|test_val| sum += test_val.0.abs_diff(*test_val.1));
    println!("{}", &contents);
    println!("{}", &sum);


    let mut times_it_appears: HashMap<u32, u32> = HashMap::new();
    for val in second {
        if times_it_appears.get(&val).is_none() {
            times_it_appears.insert(val, 0);
        }
        times_it_appears.insert(val, times_it_appears.get(&val).unwrap() + 1);
    }
    let mut sum = 0;
    for val in first {
        sum += val * times_it_appears.get(&val).unwrap_or(&0);
    }
    println!("{}", sum);
}
