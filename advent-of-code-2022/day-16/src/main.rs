use lazy_static::lazy_static;
use petgraph::{algo::astar, Graph};
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
    opened_valves: Vec<&'a str>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct StateElephant<'a> {
    path_idx: usize,
    path: Vec<&'a str>,
    elephant_path_idx: usize,
    path_elephant: Vec<&'a str>,
    minutes: u8,
    pressure_released: i16,
    opened_valves: Vec<&'a str>,
}

impl<'a> StateElephant<'a> {
    fn get_valve(&self) -> &'a str {
        return self.path[self.path_idx];
    }
    fn get_elephant_valve(&self) -> &'a str {
        return self.path_elephant[self.elephant_path_idx];
    }
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
            "path_idx: {}, path: {:?}, elephant_path_idx: {}, elephant_path: {:?}, minutes: {}, pressure: {}, opened_valves: {:?}",
            self.path_idx,
            self.path,
            self.elephant_path_idx,
            self.path_elephant,
            self.minutes,
            self.pressure_released,
            self.opened_valves
        )
    }
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    first_problem(data);
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
    let start_minutes = 26;

    let valves_with_flow: Vec<&str> = pipe_network
        .node_indices()
        .filter(|idx| pipe_network[*idx].rate > 0)
        .map(|idx| pipe_network[idx].label)
        .collect();

    for valve in valves_with_flow.iter() {
        for elephant_valve in valves_with_flow.iter() {
            if valve == elephant_valve {
                continue;
            }
            let state = StateElephant {
                path_idx: 0,
                path: get_path(&pipe_network, "AA", valve),
                elephant_path_idx: 0,
                path_elephant: get_path(&pipe_network, "AA", elephant_valve),
                minutes: start_minutes,
                pressure_released: 0,
                opened_valves: Vec::<&str>::new(),
            };
            states.push_back(state);
        }
    }
    while !states.is_empty() {
        let state = states.pop_front().unwrap();
        if state.pressure_released > max_state_drain {
            max_state_drain = state.pressure_released;
        }
        if state.minutes <= 1 {
            continue;
        }
        if state.minutes <= start_minutes - 7
            && state.pressure_released <= max_state_drain * 9 / 10
        {
            continue;
        }
        let mut new_path_idx = 0;
        let mut new_elephant_path_idx = 0;
        let mut new_path = Vec::<&str>::new();
        let mut new_elephant_path = Vec::<&str>::new();
        let mut new_pressure_released = 0;
        let mut new_opened_valves = state.opened_valves.clone();
        if state.path.len() > 0 {
            if &state.get_valve() != state.path.last().unwrap() {
                new_path_idx = state.path_idx + 1;
            } else {
                new_path_idx = 0;
                new_pressure_released +=
                    pipe_network[get_valve_by_label(&pipe_network, state.get_valve())].rate
                        * (state.minutes - 1) as i16;
                new_opened_valves.push(state.get_valve());
            }
        }
        if state.path_elephant.len() > 0 {
            if &state.get_elephant_valve() != state.path_elephant.last().unwrap() {
                new_elephant_path_idx = state.elephant_path_idx + 1;
            } else {
                new_elephant_path_idx = 0;
                new_pressure_released += pipe_network
                    [get_valve_by_label(&pipe_network, state.get_elephant_valve())]
                .rate
                    * (state.minutes - 1) as i16;
                new_opened_valves.push(state.get_elephant_valve());
            }
        }
        let potential_targets: Vec<&&str> = valves_with_flow
            .iter()
            .filter(|valve| {
                !new_opened_valves.contains(valve)
                    && (state.path.len() == 0 || *valve != state.path.last().unwrap())
                    && (state.path_elephant.len() == 0
                        || *valve != state.path_elephant.last().unwrap())
            })
            .collect();
        // println!("{:?}", potential_targets.len());
        match potential_targets.len() {
            0 => {
                if state.path.len() > 0 {
                    if !new_opened_valves.contains(state.path.last().unwrap()) {
                        new_path = state.path.clone()
                    } else {
                        new_path = Vec::<&str>::new();
                    }
                }
                if state.path_elephant.len() > 0 {
                    if !new_opened_valves.contains(state.path_elephant.last().unwrap()) {
                        new_elephant_path = state.path_elephant.clone()
                    } else {
                        new_elephant_path = Vec::<&str>::new();
                    }
                }
                states.push_back(StateElephant {
                    path_idx: new_path_idx,
                    path: new_path,
                    elephant_path_idx: new_elephant_path_idx,
                    path_elephant: new_elephant_path,
                    minutes: state.minutes - 1,
                    pressure_released: state.pressure_released + new_pressure_released,
                    opened_valves: new_opened_valves.clone(),
                })
            }
            1 => {
                let potential_target = potential_targets.first().unwrap();
                if state.path.len() > 0 {
                    if !new_opened_valves.contains(state.path.last().unwrap()) {
                        new_path = state.path.clone()
                    } else {
                        new_path =
                            get_path(&pipe_network, state.path.last().unwrap(), &potential_target)
                    }
                }
                if state.path_elephant.len() > 0 {
                    if !new_opened_valves.contains(state.path_elephant.last().unwrap()) {
                        new_elephant_path = state.path_elephant.clone()
                    } else {
                        if &new_path.last().unwrap() != potential_target {
                            new_elephant_path = get_path(
                                &pipe_network,
                                state.path_elephant.last().unwrap(),
                                &potential_target,
                            )
                        } else {
                            new_elephant_path = Vec::<&str>::new();
                        }
                    }
                }
                states.push_back(StateElephant {
                    path_idx: new_path_idx,
                    path: new_path.clone(),
                    elephant_path_idx: new_elephant_path_idx,
                    path_elephant: new_elephant_path.clone(),
                    minutes: state.minutes - 1,
                    pressure_released: state.pressure_released + new_pressure_released,
                    opened_valves: new_opened_valves.clone(),
                })
            }
            _ => {
                for potential_target in potential_targets.iter() {
                    for potential_elephant_target in potential_targets.iter() {
                        if potential_target == potential_elephant_target {
                            continue;
                        }
                        if state.path.len() > 0 {
                            if !new_opened_valves.contains(state.path.last().unwrap()) {
                                new_path = state.path.clone()
                            } else {
                                new_path = get_path(
                                    &pipe_network,
                                    state.path.last().unwrap(),
                                    &potential_target,
                                )
                            }
                        }
                        if state.path_elephant.len() > 0 {
                            if !new_opened_valves.contains(state.path_elephant.last().unwrap()) {
                                new_elephant_path = state.path_elephant.clone()
                            } else {
                                new_elephant_path = get_path(
                                    &pipe_network,
                                    state.path_elephant.last().unwrap(),
                                    &potential_elephant_target,
                                )
                            }
                        }
                        let new_state = StateElephant {
                            path_idx: new_path_idx,
                            path: new_path.clone(),
                            elephant_path_idx: new_elephant_path_idx,
                            path_elephant: new_elephant_path.clone(),
                            minutes: state.minutes - 1,
                            pressure_released: state.pressure_released + new_pressure_released,
                            opened_valves: new_opened_valves.clone(),
                        };
                        if !states.contains(&new_state) {
                            states.push_back(new_state)
                        }
                    }
                }
            }
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
        if state.minutes <= start_minutes - 15
            && state.pressure_released <= max_state_drain * 8 / 10
        {
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

fn get_path<'a>(
    pipe_network: &'a Graph<Valve<'a>, i64>,
    start: &'a str,
    end: &'a str,
) -> Vec<&'a str> {
    let (_, path) = astar(
        &pipe_network,
        get_valve_by_label(pipe_network, start),
        |node| node == get_valve_by_label(pipe_network, end),
        |edge| *edge.weight(),
        |_node| 1,
    )
    .unwrap();
    return path
        .iter()
        .map(|idx| pipe_network[*idx].label)
        .collect::<Vec<&str>>();
}
