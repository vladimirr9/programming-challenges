use std::{fs, time::Instant};

#[derive(Debug)]
struct InputNumber {
    value: i64,
    original_pos: usize,
}

fn main() {
    first_problem();
    second_problem();
}


fn second_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let key: i64 = 811589153;

    let mut numbers: Vec<InputNumber> = Vec::new();
    for (original_pos, input_line) in data.split("\n").enumerate() {
        let value = input_line.parse::<i64>().unwrap();
        numbers.push(InputNumber {
            value: value * key,
            original_pos,
        });
    }
    let result = decrypt(numbers, 10);
    println!("{} result", result);

    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn first_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut numbers: Vec<InputNumber> = Vec::new();
    for (original_pos, input_line) in data.split("\n").enumerate() {
        let value = input_line.parse::<i64>().unwrap();
        numbers.push(InputNumber {
            value: value,
            original_pos,
        });
    }
    let result = decrypt(numbers, 1);
    println!("{} result", result);

    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn decrypt(mut numbers: Vec<InputNumber>, cycles: i64) -> i64 {
    let size = numbers.len() as i64 - 1;
    for _round in 0..cycles {
        for current in 0..numbers.len() {
            let index = numbers
                .iter()
                .position(|x| x.original_pos == current)
                .unwrap();
            let mut new_index = index as i64 + numbers[index].value;
            new_index = ((new_index % size) + size) % size;
            let number = numbers.remove(index);
            numbers.insert(new_index as usize, number);
        }
    }

    let zero_index = numbers.iter().position(|x| x.value == 0).unwrap();
    let n1 = numbers[(zero_index + 1000) % numbers.len()].value;
    let n2 = numbers[(zero_index + 2000) % numbers.len()].value;
    let n3 = numbers[(zero_index + 3000) % numbers.len()].value;
    println!("{n1}, {n2}, {n3}");
    return n1 + n2 + n3;
}
