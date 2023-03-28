use lazy_static::lazy_static;
use petgraph::{data::Build, dot::Dot, prelude::DiGraph, Graph};
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fmt, fs,
};

#[derive(Eq, Hash, PartialEq, Debug)]
struct Valve<'a> {
    label: &'a str,
    rate: i32,
}

impl fmt::Display for Valve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label: {}, rate: {}", self.label, self.rate)
    }
}

struct State<'a> {
    label: &'a str,
    minutes: u32,
    pressure_released: i32,
    opened_valves: HashSet<&'a str>,
    visited: bool,
}

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "label: {}, minutes: {}, pressure: {}, opened_valves: {:?}, visited: {}",
            self.label, self.minutes, self.pressure_released, self.opened_valves, self.visited
        )
    }
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let mut pipe_network: Graph<Valve, i64> = Graph::<Valve, i64>::new();

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
        let first_node = pipe_network
            .node_indices()
            .find(|node_index| pipe_network[*node_index].label == connection.0)
            .unwrap();
        let second_node = pipe_network
            .node_indices()
            .find(|node_index| pipe_network[*node_index].label == connection.1)
            .unwrap();
        pipe_network.add_edge(first_node, second_node, 1);
    }

    println!("{}", Dot::new(&pipe_network));

    let mut states: DiGraph<State, i64> = DiGraph::<State, i64>::new();
    states.add_node(State {
        label: "AA",
        minutes: 5,
        pressure_released: 0,
        visited: false,
        opened_valves: HashSet::new(),
    });
    while let Some(index) = states
        .node_indices()
        .find(|index| !states[*index].visited && states[*index].minutes > 0)
    {
        let state = &mut states[index];
        state.visited = true;
        if !state.opened_valves.contains(state.label) {
            let state = &states[index];
            let valve = &pipe_network[get_valve_by_label(&pipe_network, state.label)];
            let mut new_opened_valves = state.opened_valves.clone();
            new_opened_valves.insert(valve.label);
            states.add_node(State {
                label: state.label,
                minutes: state.minutes - 1,
                pressure_released: state.pressure_released + valve.rate,
                opened_valves: new_opened_valves,
                visited: false,
            });
        }
        let state = &states[index];
        let mut neighbors = pipe_network.neighbors(get_valve_by_label(&pipe_network, state.label));
        while let Some(valve_index) = neighbors.next() {
            let state = &states[index];
            let valve = &pipe_network[valve_index];
            states.add_node(State {
                label: valve.label,
                minutes: state.minutes - 1,
                pressure_released: state.pressure_released,
                opened_valves: state.opened_valves.clone(),
                visited: false,
            });
        }
    }
    states
        .node_indices()
        .filter(|index| states[*index].minutes == 0)
        .for_each(|index| println!("{}", states[index].pressure_released));
}

fn parse_line(line: &str) -> (&str, i32, Vec<&str>) {
    let rate = get_rate(line);
    let mut valve_labels = str_strip_valve_labels(line);
    let valve_label = valve_labels.pop_front().unwrap();
    return (valve_label, rate, valve_labels.into());
}

fn get_rate(s: &str) -> i32 {
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

fn get_valve_by_label<'a>(
    pipe_network: &'a Graph<Valve<'a>, i64>,
    label: &'a str,
) -> petgraph::stable_graph::NodeIndex {
    pipe_network
        .node_indices()
        .find(|pipe_index| pipe_network[*pipe_index].label == label)
        .unwrap()
}
