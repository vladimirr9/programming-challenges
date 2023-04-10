use std::{fs, time::Instant};
#[derive(Debug, Clone, Copy)]
enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    number: Option<i64>,
    operation: Option<Operation>,
    first_monkey_name: Option<String>,
    second_monkey_name: Option<String>,
}

fn main() {
    // first_problem();
    second_problem();
}

fn second_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut initial_monkeys: Vec<Monkey> = Vec::new();

    get_monkeys(data, &mut initial_monkeys);

    'outer: for human_value in 0..10000 {
        let mut monkeys = initial_monkeys.clone();

        let mut humn = monkeys
            .iter_mut()
            .find(|monkey| monkey.name == "humn")
            .unwrap();
        humn.number = Some(human_value);
        while monkeys.iter().any(|monkey| monkey.number.is_none()) {
            for idx in 0..monkeys.len() {
                let monkey = &monkeys[idx];
                if monkey.number.is_some() {
                    continue;
                }
                let first_monkey = monkeys
                    .iter()
                    .find(|m| m.name == monkey.first_monkey_name.clone().unwrap())
                    .unwrap();
                let second_monkey = monkeys
                    .iter()
                    .find(|m| m.name == monkey.second_monkey_name.clone().unwrap())
                    .unwrap();
                if first_monkey.number.is_none() || second_monkey.number.is_none() {
                    continue;
                }
                let val1 = first_monkey.number.unwrap();
                let val2 = second_monkey.number.unwrap();
                let mut monkey = &mut monkeys[idx];
                monkey.number = Some(get_res(monkey.operation.clone().unwrap(), val1, val2));
            }
        }
        let root = monkeys.iter().find(|monkey| monkey.name == "root").unwrap();
        let first_monkey = monkeys
            .iter()
            .find(|m| m.name == root.first_monkey_name.clone().unwrap())
            .unwrap();
        let second_monkey = monkeys
            .iter()
            .find(|m| m.name == root.second_monkey_name.clone().unwrap())
            .unwrap();
        if first_monkey.number.unwrap() == second_monkey.number.unwrap() {
            println!("{}", human_value);
            break 'outer;
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn get_monkeys(data: &str, monkeys: &mut Vec<Monkey>) {
    for line in data.split("\n") {
        let (name, expression) = line
            .split_once(":")
            .map(|vals| (vals.0.trim(), vals.1.trim()))
            .unwrap();
        let name = String::from(name);
        if expression.parse::<i64>().is_ok() {
            let number = expression.parse::<i64>().unwrap();
            monkeys.push(Monkey {
                name: name,
                number: Some(number),
                operation: None,
                first_monkey_name: None,
                second_monkey_name: None,
            })
        } else {
            // println!("{}", expression);
            let mut vals = expression.split_whitespace();
            let first_monkey_name = vals.next().unwrap();
            let operation = match vals.next().unwrap() {
                "+" => Operation::ADD,
                "-" => Operation::SUBTRACT,
                "*" => Operation::MULTIPLY,
                "/" => Operation::DIVIDE,
                _ => panic!(),
            };
            let second_monkey_name = vals.next().unwrap();
            monkeys.push(Monkey {
                name: name,
                number: None,
                operation: Some(operation),
                first_monkey_name: Some(String::from(first_monkey_name)),
                second_monkey_name: Some(String::from(second_monkey_name)),
            })
        }
        // println!("{:?}", monkeys);
    }
}

fn first_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in data.split("\n") {
        let (name, expression) = line
            .split_once(":")
            .map(|vals| (vals.0.trim(), vals.1.trim()))
            .unwrap();
        let name = String::from(name);
        if expression.parse::<i64>().is_ok() {
            let number = expression.parse::<i64>().unwrap();
            monkeys.push(Monkey {
                name: name,
                number: Some(number),
                operation: None,
                first_monkey_name: None,
                second_monkey_name: None,
            })
        } else {
            // println!("{}", expression);
            let mut vals = expression.split_whitespace();
            let first_monkey_name = vals.next().unwrap();
            let operation = match vals.next().unwrap() {
                "+" => Operation::ADD,
                "-" => Operation::SUBTRACT,
                "*" => Operation::MULTIPLY,
                "/" => Operation::DIVIDE,
                _ => panic!(),
            };
            let second_monkey_name = vals.next().unwrap();
            monkeys.push(Monkey {
                name: name,
                number: None,
                operation: Some(operation),
                first_monkey_name: Some(String::from(first_monkey_name)),
                second_monkey_name: Some(String::from(second_monkey_name)),
            })
        }
        // println!("{:?}", monkeys);
    }
    while monkeys.iter().any(|monkey| monkey.number.is_none()) {
        for idx in 0..monkeys.len() {
            let monkey = &monkeys[idx];
            if monkey.number.is_some() {
                continue;
            }
            let first_monkey = monkeys
                .iter()
                .find(|m| m.name == monkey.first_monkey_name.clone().unwrap())
                .unwrap();
            let second_monkey = monkeys
                .iter()
                .find(|m| m.name == monkey.second_monkey_name.clone().unwrap())
                .unwrap();
            if first_monkey.number.is_none() || second_monkey.number.is_none() {
                continue;
            }
            let val1 = first_monkey.number.unwrap();
            let val2 = second_monkey.number.unwrap();
            let mut monkey = &mut monkeys[idx];
            monkey.number = Some(get_res(monkey.operation.clone().unwrap(), val1, val2));
        }
    }
    let root = monkeys.iter().find(|monkey| monkey.name == "root").unwrap();
    println!("{}", root.number.unwrap());
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn get_res(operation: Operation, val1: i64, val2: i64) -> i64 {
    match operation {
        Operation::ADD => val1 + val2,
        Operation::SUBTRACT => val1 - val2,
        Operation::MULTIPLY => val1 * val2,
        Operation::DIVIDE => val1 / val2,
    }
}
