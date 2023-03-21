use lazy_static::lazy_static;
use petgraph::{Graph, dot::Dot};
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs, fmt,
};

#[derive(Eq, Hash, PartialEq, Debug)]
struct Valve<'a> {
    label: &'a str,
    rate: i64,
}

impl fmt::Display for Valve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label: {}, rate: {}", self.label, self.rate)
    }
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut pipe_network : Graph<Valve, i64> = Graph::<Valve, i64>::new();

    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut valves: HashSet<Valve> = HashSet::new();
    println!("{}", data);
    for line in data.split("\n") {
        let (valve_label, rate, end_destinations) = parse_line(line);
        valves.insert(Valve {
            label: valve_label,
            rate: rate,
        });
        for end_destination in end_destinations {  
            connections.insert((valve_label, end_destination));
        }
    }

    
    for valve in valves {
        pipe_network.add_node(valve);
    }

    for connection in connections {
        let first_node = pipe_network.node_indices().find(|node_index| pipe_network[*node_index].label == connection.0).unwrap();
        let second_node = pipe_network.node_indices().find(|node_index| pipe_network[*node_index].label == connection.1).unwrap();
        pipe_network.add_edge(first_node, second_node, 1);
    }

    println!("{}",  Dot::new(&pipe_network));




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
    return RE.find_iter(line).map(|item| item.as_str()).collect();
}
