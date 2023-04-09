use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, fs, time::Instant};
#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_cost_ore: u16,
    clay_cost_ore: u16,
    obsidian_cost_ore: u16,
    obsidian_cost_clay: u16,
    geode_cost_ore: u16,
    geode_cost_obsidian: u16,
}
#[derive(Debug)]
struct State {
    minutes: u8,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,

    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

fn main() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in data.split("\n") {
        let values = str_strip_numbers(line);
        blueprints.push(Blueprint {
            id: values[0] as u8,
            ore_cost_ore: values[1],
            clay_cost_ore: values[2],
            obsidian_cost_ore: values[3],
            obsidian_cost_clay: values[4],
            geode_cost_ore: values[5],
            geode_cost_obsidian: values[6],
        })
    }

    for blueprint in blueprints.iter() {
        // println!("{:?}", blueprint);
        let mut states: VecDeque<State> = VecDeque::new();
        let mut most_geodes = 0;
        let starting_minutes = 24;
        let cutoff: u8 = 0;
        states.push_back(State {
            minutes: starting_minutes,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        });
        while !states.is_empty() {
            let state = states.pop_front().unwrap();
            // println!("{:?}", &state);
            if state.geode >= most_geodes {
                most_geodes = state.geode;
            }
            if state.minutes <= 0 {
                continue;
            }
            if state.minutes <= cutoff && state.clay_robots == 0 {
                continue;
            }
            if state.ore >= blueprint.ore_cost_ore {
                states.push_front(State {
                    minutes: state.minutes - 1,
                    ore: state.ore - blueprint.ore_cost_ore + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian + state.obsidian_robots,
                    geode: state.geode + state.geode_robots,
                    ore_robots: state.ore_robots + 1,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
            if state.ore >= blueprint.clay_cost_ore {
                states.push_front(State {
                    minutes: state.minutes - 1,
                    ore: state.ore - blueprint.clay_cost_ore + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian + state.obsidian_robots,
                    geode: state.geode + state.geode_robots,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots + 1,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
            if state.ore >= blueprint.obsidian_cost_ore && state.clay >= blueprint.obsidian_cost_clay {
                states.push_front(State {
                    minutes: state.minutes - 1,
                    ore: state.ore - blueprint.obsidian_cost_ore + state.ore_robots,
                    clay: state.clay - blueprint.obsidian_cost_clay + state.clay_robots,
                    obsidian: state.obsidian + state.obsidian_robots,
                    geode: state.geode + state.geode_robots,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots + 1,
                    geode_robots: state.geode_robots,
                });
            }
            if state.ore >= blueprint.geode_cost_ore && state.obsidian >= blueprint.geode_cost_obsidian {
                states.push_front(State {
                    minutes: state.minutes - 1,
                    ore: state.ore - blueprint.geode_cost_ore + state.ore_robots,
                    clay: state.clay + state.clay_robots,
                    obsidian: state.obsidian - blueprint.geode_cost_obsidian + state.obsidian_robots,
                    geode: state.geode + state.geode_robots,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots + 1,
                });
            }
            states.push_front(State {
                minutes: state.minutes - 1,
                ore: state.ore + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geode: state.geode + state.geode_robots,
                ore_robots: state.ore_robots,
                clay_robots: state.clay_robots,
                obsidian_robots: state.obsidian_robots,
                geode_robots: state.geode_robots,
            });

        }
        println!("Blueprint: {}, most geodes: {}", blueprint.id, most_geodes);
    }

    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn str_strip_numbers(s: &str) -> Vec<u16> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}
