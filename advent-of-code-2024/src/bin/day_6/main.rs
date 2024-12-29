use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::parsing_utils;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let content = parsing_utils::get_contents(6, false, 1);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
    let mut guard_pos = get_guard_position(&matrix);
    let mut visited_locations: HashSet<(usize, usize)> = HashSet::new();
    visited_locations.insert(guard_pos);
    let mut direction = Direction::UP;
    loop {
        let movement = direction.get_movement();
        let next_x = guard_pos.0 as i32 + movement.0;
        let next_y = guard_pos.1 as i32 + movement.1;
        if next_x < 0 || next_x >= height as i32 {
            break;
        }
        if next_y < 0 || next_y >= width as i32 {
            break;
        }
        let next_x = next_x as usize;
        let next_y = next_y as usize;
        if matrix[next_x][next_y] == '#' {
            direction = direction.get_next_direction();
            continue;
        }
        guard_pos = (next_x, next_y);
        visited_locations.insert(guard_pos);
    }

    println!("{}", visited_locations.len());
}

fn second_problem() {
    let content = parsing_utils::get_contents(6, false, 1);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
    let mut guard_pos = get_guard_position(&matrix);
    let mut direction = Direction::UP;
    let mut sum = 0;
    let mut state_tracker: HashSet<(usize, usize)> = HashSet::new();
    loop {
        state_tracker.insert((guard_pos.0, guard_pos.1));
        let movement = direction.get_movement();
        let next_x = guard_pos.0 as i32 + movement.0;
        let next_y = guard_pos.1 as i32 + movement.1;
        if next_x < 0 || next_x >= height as i32 {
            break;
        }
        if next_y < 0 || next_y >= width as i32 {
            break;
        }
        let next_x = next_x as usize;
        let next_y = next_y as usize;
        if matrix[next_x][next_y] == '#' {
            direction = direction.get_next_direction();
            continue;
        }
        let mut alternate_matrix = matrix.clone();
        if !state_tracker.contains(&(next_x, next_y)) {
            alternate_matrix[next_x][next_y] = '#';
            let movement_state = MovementState {
                x: guard_pos.0,
                y: guard_pos.1,
                direction: direction.clone(),
            };
            if is_loop(&alternate_matrix, movement_state) {
                sum += 1;
            }
        }
        guard_pos = (next_x, next_y);
    }

    println!("{}", sum);
}

fn is_loop(matrix: &Vec<Vec<char>>, movement_state: MovementState) -> bool {
    let mut state_tracker: HashSet<MovementState> = HashSet::new();
    state_tracker.insert(movement_state.clone());
    let height = matrix.len();
    let width = matrix[0].len();
    let mut direction = movement_state.direction.clone();
    let mut guard_pos = (movement_state.x, movement_state.y);
    loop {
        let movement = direction.get_movement();
        let next_x = guard_pos.0 as i32 + movement.0;
        let next_y = guard_pos.1 as i32 + movement.1;
        if next_x < 0 || next_x >= height as i32 {
            return false;
        }
        if next_y < 0 || next_y >= width as i32 {
            return false;
        }
        let next_x = next_x as usize;
        let next_y = next_y as usize;
        if matrix[next_x][next_y] == '#' {
            direction = direction.get_next_direction();
            let new_movement_state = MovementState {
                x: guard_pos.0,
                y: guard_pos.1,
                direction: direction.clone(),
            };
            if state_tracker.contains(&new_movement_state) {
                return true;
            }
            state_tracker.insert(new_movement_state);
            continue;
        }
        guard_pos = (next_x, next_y);
        let new_movement_state = MovementState {
            x: guard_pos.0,
            y: guard_pos.1,
            direction: direction.clone(),
        };
        if state_tracker.contains(&new_movement_state) {
            return true;
        }
        state_tracker.insert(new_movement_state);
    }
}

#[derive(PartialEq, Hash, Eq, Clone)]
struct MovementState {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(PartialEq, Hash, Eq, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn get_next_direction(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

fn get_guard_position(matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let height = matrix.len();
    let width = matrix[0].len();
    for i in 0..height {
        for j in 0..width {
            if matrix[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!()
}

fn print_matrix(matrix: &Vec<Vec<char>>, state_stracker: &HashSet<MovementState>) {
    println!("=====================");
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            let movement_state_1 = MovementState {
                x: i,
                y: j,
                direction: Direction::UP,
            };
            let movement_state_2 = MovementState {
                x: i,
                y: j,
                direction: Direction::RIGHT,
            };
            let movement_state_3 = MovementState {
                x: i,
                y: j,
                direction: Direction::DOWN,
            };
            let movement_state_4 = MovementState {
                x: i,
                y: j,
                direction: Direction::LEFT,
            };
            if (state_stracker.contains(&movement_state_1)
                || state_stracker.contains(&movement_state_3))
                && (state_stracker.contains(&movement_state_2)
                    || state_stracker.contains(&movement_state_4))
            {
                print!("+");
            } else if (state_stracker.contains(&movement_state_1)
                || state_stracker.contains(&movement_state_3))
            {
                print!("|")
            } else if (state_stracker.contains(&movement_state_2)
                || state_stracker.contains(&movement_state_4))
            {
                print!("-")
            } else {
                print!("{}", matrix[i][j]);
            }
        }
        println!()
    }
    println!("=====================");
}
