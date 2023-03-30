use lazy_static::lazy_static;
use petgraph::{dot::Dot, Graph};
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fmt, fs,
};

#[derive(Eq, Hash, PartialEq, Debug)]
struct Valve<'a> {
    label: &'a str,
    rate: i16,
}

impl fmt::Display for Valve<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "label: {}, rate: {}", self.label, self.rate)
    }
}
#[derive(Eq, Hash, PartialEq, Debug)]
struct State<'a> {
    label: &'a str,
    minutes: u8,
    pressure_released: i16,
    opened_valves: Vec<&'a str>
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct StateElephant<'a> {
    label: &'a str,
    label_elephant: &'a str,
    minutes: u8,
    pressure_released: i16,
    opened_valves: Vec<&'a str>
}

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "label: {}, minutes: {}, pressure: {}, opened_valves: {:?}",
            self.label, self.minutes, self.pressure_released, self.opened_valves
        )
    }
}

impl fmt::Display for StateElephant<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "label: {}, label_elephant: {}, minutes: {}, pressure: {}, opened_valves: {:?}",
            self.label, self.label_elephant, self.minutes, self.pressure_released, self.opened_valves
        )
    }
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    // first_problem(data);
    second_problem(data);

}

fn second_problem(data: &str) {
    let mut pipe_network: Graph<Valve, i64> = Graph::<Valve, i64>::new();

    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut valves: HashSet<Valve> = HashSet::new();
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

    let mut states: VecDeque<StateElephant> = VecDeque::new();
    let mut max_state_drain: i16 = 0;
    let start_minutes = 30;
    states.push_back(StateElephant {
        label: "AA",
        label_elephant: "AA",
        minutes: start_minutes,
        pressure_released: 0,
        opened_valves: Vec::new(),
    });
    while !states.is_empty() {
        let state = states.pop_front().unwrap();
        if state.pressure_released > max_state_drain {
            max_state_drain = state.pressure_released;
        }
        if state.minutes <= 1 {
            continue;
        }
        if state.minutes <= start_minutes - 15 && state.pressure_released <= max_state_drain * 8 / 10 {
            continue;
        }

        let valve = &pipe_network[get_valve_by_label(&pipe_network, state.label)];
        if valve.rate != 0 && !state.opened_valves.contains(&state.label) {
            let mut new_opened_valves = state.opened_valves.clone();
            new_opened_valves.push(valve.label);

            states.push_back(StateElephant {
                label: state.label,
                label_elephant: state.label_elephant,
                minutes: state.minutes - 1,
                pressure_released: state.pressure_released
                    + valve.rate * (state.minutes as i16 - 1),
                opened_valves: new_opened_valves,
            });
        }
        let mut neighbors = pipe_network.neighbors(get_valve_by_label(&pipe_network, state.label));
        while let Some(valve_index) = neighbors.next() {
            let valve = &pipe_network[valve_index];
            let potential_value = state.pressure_released;

            let new_state = StateElephant {
                label: valve.label,
                label_elephant: valve.label,
                minutes: state.minutes - 1,
                pressure_released: potential_value,
                opened_valves: state.opened_valves.clone(),
            };
            states.push_back(new_state);
        }
    }
    println!("{}", max_state_drain);
}



fn first_problem(data: &str) {
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

    let mut states: VecDeque<State> = VecDeque::new();
    let mut max_state_drain: i16 = 0;
    let start_minutes = 30;
    states.push_back(State {
        label: "AA",
        minutes: start_minutes,
        pressure_released: 0,
        opened_valves: Vec::new(),
    });
    while !states.is_empty() {
        let state = states.pop_front().unwrap();
        if state.pressure_released > max_state_drain {
            max_state_drain = state.pressure_released;
        }
        if state.minutes <= 1 {
            continue;
        }
        if state.minutes <= start_minutes - 15 && state.pressure_released <= max_state_drain * 8 / 10 {
            continue;
        }

        let valve = &pipe_network[get_valve_by_label(&pipe_network, state.label)];
        if valve.rate != 0 && !state.opened_valves.contains(&state.label) {
            let mut new_opened_valves = state.opened_valves.clone();
            new_opened_valves.push(valve.label);

            states.push_back(State {
                label: state.label,
                minutes: state.minutes - 1,
                pressure_released: state.pressure_released
                    + valve.rate * (state.minutes as i16 - 1),
                opened_valves: new_opened_valves,
            });
        }
        let mut neighbors = pipe_network.neighbors(get_valve_by_label(&pipe_network, state.label));
        while let Some(valve_index) = neighbors.next() {
            let valve = &pipe_network[valve_index];
            let potential_value = state.pressure_released;

            let new_state = State {
                label: valve.label,
                minutes: state.minutes - 1,
                pressure_released: potential_value,
                opened_valves: state.opened_valves.clone(),
            };
            states.push_back(new_state);
        }
    }
    println!("{}", max_state_drain);
}

fn parse_line(line: &str) -> (&str, i16, Vec<&str>) {
    let rate = get_rate(line);
    let mut valve_labels = str_strip_valve_labels(line);
    let valve_label = valve_labels.pop_front().unwrap();
    return (valve_label, rate, valve_labels.into());
}

fn get_rate(s: &str) -> i16 {
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
