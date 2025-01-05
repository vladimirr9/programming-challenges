use advent_of_code_2024::utils::parsing_utils;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

const A_PRESS_COST: u32 = 3;
const B_PRESS_COST: u32 = 1;

fn main() {
    let content = parsing_utils::get_contents(13, true, 1).trim().to_owned();
    let machines: Vec<String> = content
        .split("\n\n")
        .map(|machine| machine.trim().to_owned())
        .collect();

    // for machine in machines.iter() {
    //     println!("{}", machine)
    // }

    // let regex = Regex::new(
    //     r"Button A: X\+(\d+), Y\+(\d+)\s*
    //       Button B: X\+(\d+), Y\+(\d+)\s*
    //       Prize: X=(\d+), Y=(\d+)"
    // ).unwrap();

    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\n\s*Button B: X\+(\d+), Y\+(\d+)\n\s*Prize: X=(\d+), Y=(\d+)"
    ).unwrap();


    for machine in machines {
        let res = regex.captures(machine.as_ref()).unwrap();
        let (A_x, A_y, B_x, B_y, target_x, target_y) = (
            res.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(6).unwrap().as_str().parse::<u32>().unwrap(),
        );
        let machine_config = MachineConfig{
            a_x: A_x,
            a_y: A_y,
            b_x: B_x,
            b_y: B_y,
            target_x,
            target_y,
        };
        let tokens = get_tokens_needed(State { x: 0, y: 0, a_pressed: 0, b_pressed: 0 }, &machine_config);
        println!("{:?}", tokens);
    }
}

fn get_tokens_needed(state: State, machine_config: &MachineConfig) -> Option<u32> {
    // println!("{:?}", state);
    if state.x == machine_config.target_x && state.y == machine_config.target_y {
        return Some(0);
    }
    let press_a_x = state.x + machine_config.a_x;
    let press_a_y = state.y + machine_config.a_y;
    let press_b_x = state.x + machine_config.b_x;
    let press_b_y = state.y + machine_config.b_y;

    if state.b_pressed + 1 <= 100
        && press_b_x <= machine_config.target_x
        && press_b_y <= machine_config.target_y
    {
        let new_state = State {
            x: press_b_x,
            y: press_b_y,
            a_pressed: state.a_pressed,
            b_pressed: state.b_pressed + 1,
        };
        let cost = get_tokens_needed(new_state, machine_config);
        if let Some(cost) = cost {
            return Some(B_PRESS_COST + cost);
        }
    }
    if state.a_pressed + 1 <= 100
        && press_a_x <= machine_config.target_x
        && press_a_y <= machine_config.target_y
    {
        let new_state = State {
            x: press_a_x,
            y: press_a_y,
            a_pressed: state.a_pressed + 1,
            b_pressed: state.b_pressed,
        };
        let cost = get_tokens_needed(new_state, machine_config);
        if let Some(cost) = cost {
            return Some(A_PRESS_COST + cost);
        }
    }
    return None;
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct MachineConfig {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    target_x: u32,
    target_y: u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct State {
    x: u32,
    y: u32,
    a_pressed: u32,
    b_pressed: u32,
}



#[test]
fn test() {
    let a1 = 94.0;
    let b1 = 22.0;
    let c1 = 8400.0;

    let a2 = 34.0;
    let b2 = 67.0;
    let c2 = 5400.0;

    let epsilon = 1e-3;

    
    // Create the coefficient matrix and the constants vector
    let coefficients = Matrix2::new(a1, b1, a2, b2);
    let constants = Vector2::new(c1, c2);

    // Solve for the variables (x, y)
    if let Some(solution) = coefficients.try_inverse() {
        let result = solution * constants;
        println!("Solution: x = {}, y = {}", result[0], result[1]);
    } else {
        println!("The system of equations has no unique solution.");
    }
}

fn solve_machine(machine_config: &MachineConfig) {

}