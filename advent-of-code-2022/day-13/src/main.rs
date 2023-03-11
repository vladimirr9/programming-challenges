use compare::Compare;
use std::{cmp::Ordering, fs};

#[derive(Debug)]
enum ValueType {
    Int,
    List,
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    second_problem(data);
    // first_problem(data);
}

fn second_problem(data: &str) {
    let mut packets: Vec<&str> = Vec::new();
    for row in data.split("\n") {
        if row.is_empty() {
            continue;
        }
        packets.push(row);
    }
    packets.push("[[2]]");
    packets.push("[[6]]");

    packets.sort_by(
        |packet_1, packet_2| match compare_packets(*packet_1, *packet_2) {
            true => Ordering::Less,
            false => Ordering::Greater,
        },
    );
    for packet in packets.iter() {
        println!("{}", packet);
    }
    let first_divider = packets
        .iter()
        .position(|packet| *packet == "[[2]]")
        .unwrap()
        + 1;
    let second_divider = packets
        .iter()
        .position(|packet| *packet == "[[6]]")
        .unwrap()
        + 1;
    println!("first_divider: {}", first_divider);
    println!("second_divider: {}", second_divider);
    println!("{}", first_divider * second_divider);
}

fn first_problem(data: &str) {
    let packet_pairs: Vec<&str> = data.split("\n\n").into_iter().collect();
    let mut index_sum = 0;
    // println!("{:?}", packet_pairs);
    for (i, pair) in packet_pairs.iter().enumerate() {
        let (first_pair, second_pair) = pair.split_once("\n").unwrap();
        println!("====== Packet pair {} ======", i + 1);
        println!("Packet 1: {}", first_pair);
        println!("Packet 2: {}", second_pair);
        let correct_order = compare_packets(first_pair, second_pair);
        println!("{}", correct_order);
        println!("=============");
        if correct_order {
            index_sum += i + 1;
        }
    }
    println!("Index sum: {}", index_sum);
}

fn compare_packets(first_pair: &str, second_pair: &str) -> bool {
    let first_content = first_pair.get(1..first_pair.len() - 1).unwrap();
    let second_content = second_pair.get(1..second_pair.len() - 1).unwrap();
    let mut first_pointer: usize = 0;
    let mut second_pointer: usize = 0;
    loop {
        let first_value = get_next_value(first_content, first_pointer);
        let second_value = get_next_value(second_content, second_pointer);
        if first_value.is_none() {
            return true;
        }
        if second_value.is_none() {
            return false;
        }
        let (first_value, first_type) = first_value.unwrap();
        let (second_value, second_type) = second_value.unwrap();
        first_pointer += first_value.len() + 1;
        second_pointer += second_value.len() + 1;
        match compare_values(first_value, first_type, second_value, second_type) {
            Ordering::Greater => {
                return false;
            }
            Ordering::Equal => {
                continue;
            }
            Ordering::Less => {
                return true;
            }
        };
    }
}

fn compare_lists(first_list: &str, second_list: &str) -> Ordering {
    let first_content = first_list.get(1..first_list.len() - 1).unwrap();
    let second_content = second_list.get(1..second_list.len() - 1).unwrap();
    let mut first_pointer: usize = 0;
    let mut second_pointer: usize = 0;
    loop {
        let first_value = get_next_value(first_content, first_pointer);
        let second_value = get_next_value(second_content, second_pointer);
        if first_value.is_none() && second_value.is_none() {
            return Ordering::Equal;
        }
        if first_value.is_none() {
            return Ordering::Less;
        }
        if second_value.is_none() {
            return Ordering::Greater;
        }
        let (first_value, first_type) = first_value.unwrap();
        let (second_value, second_type) = second_value.unwrap();
        first_pointer += first_value.len() + 1;
        second_pointer += second_value.len() + 1;
        let comparison = compare_values(first_value, first_type, second_value, second_type);
        if comparison == Ordering::Equal {
            continue;
        }
        return comparison;
    }
}

fn compare_values(
    first_value: &str,
    first_type: ValueType,
    second_value: &str,
    second_type: ValueType,
) -> Ordering {
    let cmp = compare::natural();
    println!(
        "Comparing {}, and {} of types {:?} and {:?}",
        first_value, second_value, first_type, second_type
    );
    match (first_type, second_type) {
        (ValueType::Int, ValueType::Int) => cmp.compare(
            &first_value.parse::<u32>().unwrap(),
            &second_value.parse::<u32>().unwrap(),
        ),
        (ValueType::List, ValueType::Int) => compare_values(
            first_value,
            ValueType::List,
            &format!("[{}]", second_value),
            ValueType::List,
        ),
        (ValueType::Int, ValueType::List) => compare_values(
            &format!("[{}]", first_value),
            ValueType::List,
            second_value,
            ValueType::List,
        ),
        (ValueType::List, ValueType::List) => compare_lists(first_value, second_value),
    }
}

fn get_next_value(line: &str, current_position: usize) -> Option<(&str, ValueType)> {
    if current_position >= line.len() {
        return None;
    }
    let mut end_position = current_position;
    if line.chars().nth(current_position).unwrap().is_ascii_digit() {
        while line.chars().nth(end_position).unwrap().is_ascii_digit() {
            end_position += 1;
            if end_position >= line.len() {
                break;
            }
        }
        return Some((
            line.get(current_position..end_position).unwrap(),
            ValueType::Int,
        ));
    } else if line.chars().nth(current_position).unwrap() == '[' {
        let mut open_brackets = 1;
        while open_brackets != 0 {
            end_position += 1;
            open_brackets += match line.chars().nth(end_position).unwrap() {
                '[' => 1,
                ']' => -1,
                _ => 0,
            }
        }
        return Some((
            line.get(current_position..=end_position).unwrap(),
            ValueType::List,
        ));
    }

    None
}

#[test]
fn test_compare_values() {
    let res = compare_packets("[[]]", "[[8]]");
    assert!(res)
}
