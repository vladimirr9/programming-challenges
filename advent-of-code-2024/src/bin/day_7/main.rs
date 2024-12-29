use std::collections::HashMap;

use advent_of_code_2024::utils::parsing_utils;

fn main() {
    first_problem();
    }
    
    fn first_problem() {
    let content = parsing_utils::get_contents(7, false, 1).trim().to_owned();
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();
    let mut sum = 0;
    for equation in content.lines() {
        let (test_value, numbers) = equation.split_once(":").unwrap();
        let test_value = test_value.parse::<u64>().unwrap();
        let numbers: Vec<u64> = numbers
            .trim()
            .split_ascii_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect();
        equations.push((test_value, numbers));
    }
    for equation in equations {
        let (test_value, numbers) = equation;
        let all_values = get_all_values(&numbers);
        if all_values.iter().any(|res| *res == test_value) {
            sum += test_value;
        }
    }
    println!("{sum}");
    }



fn get_all_values(numbers: &Vec<u64>) -> Vec<u64> {
    if numbers.len() <= 1 {
        panic!()
    }
    if numbers.len() == 2 {
        return vec![numbers[0] + numbers[1], numbers[0] * numbers[1]];
    }
    let left_side = get_all_values(&numbers.clone().into_iter().take(numbers.len()-1).collect());
    let mut res: Vec<u64> = Vec::new();
    for val in left_side {
        res.push(numbers[numbers.len() -1] + val);
        res.push(numbers[numbers.len() -1] * val);
    }
    return res
}
