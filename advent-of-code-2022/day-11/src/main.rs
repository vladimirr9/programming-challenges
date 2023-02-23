#![feature(get_many_mut)]
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, str::FromStr};

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test_val: i64,
    true_pass: usize,
    false_pass: usize,
    inspected_items: u64,
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    first_problem(data);
    second_problem(data);
}

fn second_problem(data: &str) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_attribute in data.split("\n\n") {
        let mut line_iter = monkey_attribute.split("\n").into_iter();
        // enumerating monkey
        line_iter.next();
        // starting items
        let val = line_iter.next().unwrap();
        let starting_items = str_strip_numbers(val);
        // operation
        let (_, expression) = line_iter.next().unwrap().split_once("=").unwrap();
        let expression = expression.trim();
        let operation_sign = if expression.contains("+") { "+" } else { "*" };
        let (_, b) = expression.split_once(operation_sign).unwrap();
        let b = String::from(b.trim());
        println!(
            "Expression: {}, operation_sign: {}, b: {}",
            expression, operation_sign, b
        );
        let operation_sign = String::from_str(operation_sign).unwrap();
        // assuming first will always be old
        let operation = move |val: i64| -> i64 {
            if operation_sign == "+" {
                if b == "old" {
                    val + val
                } else {
                    val + b.parse::<i64>().unwrap()
                }
            } else {
                if b == "old" {
                    val * val
                } else {
                    val * b.parse::<i64>().unwrap()
                }
            }
        };
        // divisible val
        let val = line_iter.next().unwrap();
        let divisible = *str_strip_numbers(val).get(0).unwrap();

        // true val
        let val = line_iter.next().unwrap();
        let true_val: usize = (*str_strip_numbers(val).get(0).unwrap())
            .try_into()
            .unwrap();
        // false val
        let val = line_iter.next().unwrap();
        let false_val: usize = (*str_strip_numbers(val).get(0).unwrap())
            .try_into()
            .unwrap();

        let monkey = Monkey {
            items: starting_items,
            operation: Box::new(operation),
            test_val: divisible,
            true_pass: true_val,
            false_pass: false_val,
            inspected_items: 0,
        };
        monkeys.push(monkey);
    }
    let lowest_common_denominator_test: i64 = monkeys
        .iter()
        .map(|monkey| monkey.test_val)
        .reduce(|first_test_val, second_test_val| {
            lowest_common_denominator(first_test_val, second_test_val)
        })
        .unwrap().into();
    for round in 0..10000 {
        println!("ROUND: {}", round + 1);
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap();
            let indices = [i, monkey.true_pass, monkey.false_pass];
            let mut fetched_monkeys = monkeys.get_many_mut(indices).unwrap();
            let mut iter = fetched_monkeys.iter_mut();
            let monkey = iter.next().unwrap();
            let monkey_true = iter.next().unwrap();
            let monkey_false = iter.next().unwrap();
            let items = &mut monkey.items;
            while !items.is_empty() {
                let item_worry = items.remove(0);
                monkey.inspected_items += 1;
                let new_item_worry = (monkey.operation)(item_worry) % lowest_common_denominator_test;
                if new_item_worry % monkey.test_val == 0 {
                    monkey_true.items.push(new_item_worry);
                } else {
                    monkey_false.items.push(new_item_worry);
                };

                println!("Monkey : {}, inspected {}", i, monkey.inspected_items);
            }
        }
    }
    monkeys.sort_by(|first_monkey, second_monkey| {
        second_monkey
            .inspected_items
            .cmp(&first_monkey.inspected_items)
    });
    println!(
        "Monkey business {}",
        monkeys[0].inspected_items * monkeys[1].inspected_items
    );
}

fn first_problem(data: &str) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_attribute in data.split("\n\n") {
        let mut line_iter = monkey_attribute.split("\n").into_iter();
        // enumerating monkey
        line_iter.next();
        // starting items
        let val = line_iter.next().unwrap();
        let starting_items = str_strip_numbers(val);
        // operation
        let (_, expression) = line_iter.next().unwrap().split_once("=").unwrap();
        let expression = expression.trim();
        let operation_sign = if expression.contains("+") { "+" } else { "*" };
        let (_, b) = expression.split_once(operation_sign).unwrap();
        let b = String::from(b.trim());
        println!(
            "Expression: {}, operation_sign: {}, b: {}",
            expression, operation_sign, b
        );
        let operation_sign = String::from_str(operation_sign).unwrap();
        // assuming first will always be old
        let operation = move |val: i64| -> i64 {
            if operation_sign == "+" {
                if b == "old" {
                    val + val
                } else {
                    val + b.parse::<i64>().unwrap()
                }
            } else {
                if b == "old" {
                    val * val
                } else {
                    val * b.parse::<i64>().unwrap()
                }
            }
        };
        // divisible val
        let val = line_iter.next().unwrap();
        let divisible = *str_strip_numbers(val).get(0).unwrap();

        // true val
        let val = line_iter.next().unwrap();
        let true_val: usize = (*str_strip_numbers(val).get(0).unwrap())
            .try_into()
            .unwrap();
        // false val
        let val = line_iter.next().unwrap();
        let false_val: usize = (*str_strip_numbers(val).get(0).unwrap())
            .try_into()
            .unwrap();

        let monkey = Monkey {
            items: starting_items,
            operation: Box::new(operation),
            test_val: divisible,
            true_pass: true_val,
            false_pass: false_val,
            inspected_items: 0,
        };
        monkeys.push(monkey);
    }
    for round in 0..20 {
        println!("ROUND: {}", round + 1);
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap();
            let indices = [i, monkey.true_pass, monkey.false_pass];
            let mut fetched_monkeys = monkeys.get_many_mut(indices).unwrap();
            let mut iter = fetched_monkeys.iter_mut();
            let monkey = iter.next().unwrap();
            let monkey_true = iter.next().unwrap();
            let monkey_false = iter.next().unwrap();
            let items = &mut monkey.items;
            while !items.is_empty() {
                let item_worry = items.remove(0);
                monkey.inspected_items += 1;
                let mut new_item_worry = (monkey.operation)(item_worry);
                new_item_worry /= 3;
                if new_item_worry % monkey.test_val == 0 {
                    monkey_true.items.push(new_item_worry);
                } else {
                    monkey_false.items.push(new_item_worry);
                };

                println!("Monkey : {}, inspected {}", i, monkey.inspected_items);
            }
        }
    }
    monkeys.sort_by(|first_monkey, second_monkey| {
        second_monkey
            .inspected_items
            .cmp(&first_monkey.inspected_items)
    });
    println!(
        "Monkey business {}",
        monkeys[0].inspected_items * monkeys[1].inspected_items
    );
}

fn lowest_common_denominator(a: i64, b: i64) -> i64 {
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    return a;
}

fn str_strip_numbers(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}
