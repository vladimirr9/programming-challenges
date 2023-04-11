use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::Mutex,
    time::Instant,
};

use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
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
    // first_problem();
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

    let counter: u64 = 0;
    let mutex = Mutex::new(counter);
    let affected_by_humn = affected_by_humn.into_iter().collect::<Vec<String>>();

    let mut modified_monkeys = monkeys.clone();
    for name in affected_by_humn.iter() {
        modified_monkeys.insert(
            String::from(name),
            initial_monkeys.get_mut(name).unwrap().clone(),
        );
    }

    (0..100_000_000i64).into_par_iter().for_each(|human_value| {
        let mut monkeys = modified_monkeys.clone();

        let mut humn = monkeys.get_mut("humn").unwrap();
        humn.number = Some(human_value);
        while monkeys.get("root").unwrap().number.is_none() {
            for key in affected_by_humn.iter() {
                let monkey = monkeys.get(key).unwrap();
                if monkey.number.is_some() {
                    continue;
                }
                let first_monkey = monkeys
                    .get(*&monkey.first_monkey_name.as_ref().unwrap())
                    .unwrap();
                if first_monkey.number.is_none() {
                    continue;
                }
                let second_monkey = monkeys
                    .get(*&monkey.second_monkey_name.as_ref().unwrap())
                    .unwrap();
                if second_monkey.number.is_none() {
                    continue;
                }
                let val1 = first_monkey.number.unwrap();
                let val2 = second_monkey.number.unwrap();
                let mut monkey = monkeys.get_mut(key).unwrap();
                monkey.number = Some(get_res(monkey.operation.unwrap(), val1, val2));
            }
        }
        // println!("{}", affected_by_humn.len());
        let root = monkeys.get("root").unwrap();
        let first_monkey = monkeys
            .get(&root.first_monkey_name.clone().unwrap())
            .unwrap();
        let second_monkey = monkeys
            .get(&root.second_monkey_name.clone().unwrap())
            .unwrap();
        if first_monkey.number.unwrap() == second_monkey.number.unwrap() {
            println!("======={}======", human_value);
        }
        let mut val = mutex.lock().unwrap();
        *val += 1;
        if *val % 1_000_000 == 0 {
            println!("{val}");
        }
    });

    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
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

// fn first_problem() {
//     let now = Instant::now();
//     let filepath = "input.txt";
//     let data = fs::read_to_string(filepath).expect("Should be able to read file");
//     let data = data.trim();

//     let mut monkeys: Vec<Monkey> = Vec::new();

//     for line in data.split("\n") {
//         let (name, expression) = line
//             .split_once(":")
//             .map(|vals| (vals.0.trim(), vals.1.trim()))
//             .unwrap();
//         let name = String::from(name);
//         if expression.parse::<i64>().is_ok() {
//             let number = expression.parse::<i64>().unwrap();
//             monkeys.push(Monkey {
//                 name: name,
//                 number: Some(number),
//                 operation: None,
//                 first_monkey_name: None,
//                 second_monkey_name: None,
//             })
//         } else {
//             // println!("{}", expression);
//             let mut vals = expression.split_whitespace();
//             let first_monkey_name = vals.next().unwrap();
//             let operation = match vals.next().unwrap() {
//                 "+" => Operation::ADD,
//                 "-" => Operation::SUBTRACT,
//                 "*" => Operation::MULTIPLY,
//                 "/" => Operation::DIVIDE,
//                 _ => panic!(),
//             };
//             let second_monkey_name = vals.next().unwrap();
//             monkeys.push(Monkey {
//                 name: name,
//                 number: None,
//                 operation: Some(operation),
//                 first_monkey_name: Some(String::from(first_monkey_name)),
//                 second_monkey_name: Some(String::from(second_monkey_name)),
//             })
//         }
//         // println!("{:?}", monkeys);
//     }
//     while monkeys.iter().any(|monkey| monkey.number.is_none()) {
//         for idx in 0..monkeys.len() {
//             let monkey = &monkeys[idx];
//             if monkey.number.is_some() {
//                 continue;
//             }
//             let first_monkey = monkeys
//                 .iter()
//                 .find(|m| m.name == monkey.first_monkey_name.clone().unwrap())
//                 .unwrap();
//             let second_monkey = monkeys
//                 .iter()
//                 .find(|m| m.name == monkey.second_monkey_name.clone().unwrap())
//                 .unwrap();
//             if first_monkey.number.is_none() || second_monkey.number.is_none() {
//                 continue;
//             }
//             let val1 = first_monkey.number.unwrap();
//             let val2 = second_monkey.number.unwrap();
//             let mut monkey = &mut monkeys[idx];
//             monkey.number = Some(get_res(monkey.operation.clone().unwrap(), val1, val2));
//         }
//     }
//     let root = monkeys.iter().find(|monkey| monkey.name == "root").unwrap();
//     println!("{}", root.number.unwrap());
//     let elapsed = now.elapsed();
//     println!("Elapsed part 1: {:.2?}", elapsed);
// }

fn get_res(operation: Operation, val1: i64, val2: i64) -> i64 {
    match operation {
        Operation::ADD => val1 + val2,
        Operation::SUBTRACT => val1 - val2,
        Operation::MULTIPLY => val1 * val2,
        Operation::DIVIDE => val1 / val2,
    }
}
