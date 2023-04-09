use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{collections::VecDeque, fs, time::Instant, sync::Mutex};
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
    // first_problem();
    second_problem();
}

fn second_problem() {
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
    let results: Vec<u16> = vec![0; blueprints.len()];
    let mutex_results = Mutex::new(results);

    blueprints.par_iter().for_each(|blueprint| {
        let blueprint_now = Instant::now();
        // println!("{:?}", blueprint);
        let mut states: VecDeque<State> = VecDeque::new();
        let mut most_geodes = 0;
        let starting_minutes = 30;

        let blueprint_max_ore_cost = *[blueprint.ore_cost_ore, blueprint.clay_cost_ore, blueprint.obsidian_cost_ore, blueprint.geode_cost_ore].iter().max_by(|a,b| a.cmp(b)).unwrap();
        let blueprint_max_clay_cost = blueprint.obsidian_cost_clay;
        let blueprint_max_obsidian_cost = blueprint.geode_cost_obsidian;

        let cutoff: u8 = 26;
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
            if state.minutes <= (starting_minutes - cutoff) && ((state.geode == 0) || (state.ore + state.clay + state.obsidian > 60) || (state.ore_robots == 1)) {
                continue;
            }
            if state.ore >= blueprint.ore_cost_ore && state.ore_robots < blueprint_max_ore_cost {
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
            if state.ore >= blueprint.clay_cost_ore && state.clay_robots < blueprint_max_clay_cost {
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
            if state.ore >= blueprint.obsidian_cost_ore && state.clay >= blueprint.obsidian_cost_clay && state.obsidian_robots < blueprint_max_obsidian_cost {
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
        let elapsed_blueprint = blueprint_now.elapsed();
        println!("Blueprint: {}, most geodes: {}", blueprint.id, most_geodes);
        println!("Elapsed blueprint {}: {:.2?}", blueprint.id, elapsed_blueprint);
        let mut vector = mutex_results.lock().unwrap();
        vector[(blueprint.id - 1) as usize] = (blueprint.id as u16) * most_geodes;
    });

    let final_res = mutex_results.lock().unwrap();
    println!("{:?}", final_res);
    println!("{:?}", final_res.iter().sum::<u16>());
    let elapsed = now.elapsed();
    println!("Elapsed part 2: {:.2?}", elapsed);
}


// runs for a long time (3000s on my PC) but gets the correct result
fn first_problem() {
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
    let results: Vec<u16> = vec![0; blueprints.len()];
    let mutex_results = Mutex::new(results);

    blueprints.par_iter().for_each(|blueprint| {
        let blueprint_now = Instant::now();
        // println!("{:?}", blueprint);
        let mut states: VecDeque<State> = VecDeque::new();
        let mut most_geodes = 0;
        let starting_minutes = 24;
        // let cutoff: u8 = 0;
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
            // if state.minutes <= cutoff && (state.obsidian_robots == 0 || state.clay_robots == 0 || state.ore_robots == 1) {
            //     continue;
            // }
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
        let elapsed_blueprint = blueprint_now.elapsed();
        println!("Blueprint: {}, most geodes: {}", blueprint.id, most_geodes);
        println!("Elapsed blueprint {}: {:.2?}", blueprint.id, elapsed_blueprint);
        let mut vector = mutex_results.lock().unwrap();
        vector[(blueprint.id - 1) as usize] = (blueprint.id as u16) * most_geodes;
    });

    let final_res = mutex_results.lock().unwrap();
    println!("{:?}", final_res);
    println!("{:?}", final_res.iter().sum::<u16>());
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
