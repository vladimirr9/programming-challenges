use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

#[derive(Debug, Clone, Copy)]
enum Operation {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

#[derive(Debug, Clone)]
struct Monkey {
    number: Option<i64>,
    operation: Option<Operation>,
    first_monkey_name: Option<String>,
    second_monkey_name: Option<String>,
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

    let mut initial_monkeys: HashMap<String, Monkey> = HashMap::new();

    get_monkeys(data, &mut initial_monkeys);

    let mut keys: Vec<String> = Vec::new();
    for (name, _) in initial_monkeys.iter() {
        keys.push(String::from(name));
    }
    let mut affected_by_humn: HashSet<String> = HashSet::new();
    affected_by_humn.insert(String::from("humn"));

    // (0..10i64).into_par_iter().for_each(|human_value| {
    let mut monkeys = initial_monkeys.clone();
    (0..1i64).into_iter().for_each(|human_value| {
        let mut humn = monkeys.get_mut("humn").unwrap();
        humn.number = Some(human_value);
        while monkeys.iter().any(|pair| pair.1.number.is_none()) {
            for key in keys.iter() {
                let monkey = monkeys.get(key).unwrap();
                if monkey.number.is_some() {
                    continue;
                }
                if affected_by_humn.contains(&*monkey.first_monkey_name.as_ref().unwrap())
                    || affected_by_humn.contains(&*monkey.second_monkey_name.as_ref().unwrap())
                {
                    affected_by_humn.insert(String::from(key.clone()));
                }

                let first_monkey = monkeys
                    .get(&monkey.first_monkey_name.clone().unwrap())
                    .unwrap();
                let second_monkey = monkeys
                    .get(&monkey.second_monkey_name.clone().unwrap())
                    .unwrap();

                if first_monkey.number.is_none() || second_monkey.number.is_none() {
                    continue;
                }
                let val1 = first_monkey.number.unwrap();
                let val2 = second_monkey.number.unwrap();
                let mut monkey = monkeys.get_mut(key).unwrap();
                monkey.number = Some(get_res(monkey.operation.clone().unwrap(), val1, val2));
            }
        }
    });

    let affected_by_humn = affected_by_humn.into_iter().collect::<Vec<String>>();

    let mut modified_monkeys = monkeys.clone();
    for name in affected_by_humn.iter() {
        modified_monkeys.insert(
            String::from(name),
            initial_monkeys.get_mut(name).unwrap().clone(),
        );
    }

    let root_monkey = modified_monkeys.get("root").unwrap();
    let mut expression: String = "(".to_owned()
        + &root_monkey.first_monkey_name.clone().unwrap()
        + " = "
        + &root_monkey.second_monkey_name.clone().unwrap()
        + ")";

    while str_strip_name(&expression)
        .iter()
        .any(|name| *name != "humn")
    {
        let name = *str_strip_name(&expression)
            .iter()
            .find(|name| **name != "humn")
            .unwrap();
        let monkey = modified_monkeys.get(name).unwrap();
        let replacement: String = if monkey.number.is_some() {
            monkey.number.unwrap().to_string()
        } else {
            "(".to_owned()
                + &monkey.first_monkey_name.clone().unwrap()
                + " "
                + &get_operation(monkey.operation.unwrap())
                + " "
                + &monkey.second_monkey_name.clone().unwrap()
                + ")"
        };
        expression = expression.replace(name, &replacement);
    }

    println!("{expression}");

    let elapsed = now.elapsed();
    println!("Elapsed part 2: {:.2?}", elapsed);
}

fn get_monkeys(data: &str, monkeys: &mut HashMap<String, Monkey>) {
    for line in data.split("\n") {
        let (name, expression) = line
            .split_once(":")
            .map(|vals| (vals.0.trim(), vals.1.trim()))
            .unwrap();
        let name = String::from(name);
        if expression.parse::<i64>().is_ok() {
            let number = expression.parse::<i64>().unwrap();
            monkeys.insert(
                name,
                Monkey {
                    number: Some(number),
                    operation: None,
                    first_monkey_name: None,
                    second_monkey_name: None,
                },
            );
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
            monkeys.insert(
                name,
                Monkey {
                    number: None,
                    operation: Some(operation),
                    first_monkey_name: Some(String::from(first_monkey_name)),
                    second_monkey_name: Some(String::from(second_monkey_name)),
                },
            );
        }
        // println!("{:?}", monkeys);
    }
}

fn first_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    get_monkeys(data, &mut monkeys);

    let mut keys: Vec<String> = Vec::new();
    for (name, _) in monkeys.iter() {
        keys.push(String::from(name));
    }

    while monkeys.iter().any(|pair| pair.1.number.is_none()) {
        for key in keys.iter() {
            let monkey = monkeys.get(key).unwrap();
            if monkey.number.is_some() {
                continue;
            }
            let first_monkey = monkeys
                .get(&monkey.first_monkey_name.clone().unwrap())
                .unwrap();
            let second_monkey = monkeys
                .get(&monkey.second_monkey_name.clone().unwrap())
                .unwrap();

            if first_monkey.number.is_none() || second_monkey.number.is_none() {
                continue;
            }
            let val1 = first_monkey.number.unwrap();
            let val2 = second_monkey.number.unwrap();
            let mut monkey = monkeys.get_mut(key).unwrap();
            monkey.number = Some(get_res(monkey.operation.clone().unwrap(), val1, val2));
        }
    }
    let root = monkeys.get("root").unwrap();
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

fn get_operation(operation: Operation) -> String {
    match operation {
        Operation::ADD => "+".to_owned(),
        Operation::SUBTRACT => "-".to_owned(),
        Operation::MULTIPLY => "*".to_owned(),
        Operation::DIVIDE => "/".to_owned(),
    }
}

fn str_strip_name(s: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]){4}").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|res| Some(res.as_str()))
        .collect()
}
