use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding;
    let (stacks, instructions) = data.split_once("\n\n").expect("Splits at middle");

    let instructions = instructions.trim();
    let mut stacks = get_stacks(stacks);

    for instruction in instructions.split("\n") {
        let (times, from, to) = get_instruction_values(instruction);
        let from = from - 1;
        let to = to - 1;
        for _i in 0..times {
            let origin_stack = &mut stacks[from];
            let char = origin_stack.pop().unwrap();
            let destination_stack = &mut stacks[to];
            destination_stack.push(char);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}

fn get_stacks(stacks: &str) -> Vec<Vec<char>> {
    let num_of_stacks = stacks.split("\n").last().unwrap();
    let length = num_of_stacks
        .chars()
        .position(|ele| ele == '2')
        .expect("There must be at least 2 stacks")
        - 1;
    let stack_rows: Vec<&str> = stacks
        .split("\n")
        .filter(|row| *row != num_of_stacks)
        .collect();
    let num_of_stacks = str_strip_numbers(num_of_stacks).len();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _i in 0..num_of_stacks {
        stacks.push(Vec::new())
    }

    for row in stack_rows.iter().rev() {
        for i in 0..stacks.len() {
            let char = row.chars().nth(1 + i * length).unwrap();
            if char.is_alphabetic() {
                let stack = stacks.get_mut(i).unwrap();
                stack.push(char);
            }
        }
    }
    return stacks;
}

fn str_strip_numbers(s: &str) -> Vec<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

fn get_instruction_values(instruction: &str) -> (usize, usize, usize) {
    let numbers = str_strip_numbers(instruction);
    return (
        *numbers.get(0).unwrap() as usize,
        *numbers.get(1).unwrap() as usize,
        *numbers.get(2).unwrap() as usize,
    );
}
