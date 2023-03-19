use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, collections::VecDeque};

struct Valve<'a> {
    label: &'a str,
    rate: i64,
    connected_nodes: Vec<&'a str>
}



fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    println!("{}", data);
    for line in data.split("\n") {
        println!("{:?}", parse_line(line));
    }
}


fn parse_line(line: &str) -> (&str, i64, Vec<&str>) {
    let rate = get_rate(line);
    let mut valve_labels = str_strip_valve_labels(line);
    let valve_label = valve_labels.pop_front().unwrap();
    return (valve_label, rate, valve_labels.into());

}


fn get_rate(s: &str) -> i64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    return RE
        .find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .next()
        .unwrap();
}


fn str_strip_valve_labels(line: &str) -> VecDeque<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[A-Z]{2}").unwrap();
    }
    return RE
        .find_iter(line).map(|item| item.as_str())
        .collect();
}