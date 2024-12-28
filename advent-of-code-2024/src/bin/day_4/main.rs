use advent_of_code_2024::utils::parsing_utils;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let contents = parsing_utils::get_contents(4, false, 1).trim().to_owned();
    let matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = matrix.len();
    let width = matrix[0].len();

    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            if matrix[i][j] == 'X' {
                sum += count_xmas(&matrix, i, j);
            }
        }
    }
    print!("{sum}")
}

fn second_problem() {
    let contents = parsing_utils::get_contents(4, false, 2).trim().to_owned();
    let matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = matrix.len();
    let width = matrix[0].len();

    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            if matrix[i][j] == 'A' {
                if i as i32 - 1 < 0 {
                    continue;
                }
                if i as i32 + 1 >= height as i32 {
                    continue;
                }
                if j as i32 - 1 < 0 {
                    continue;
                }
                if j as i32 + 1 >= width as i32 {
                    continue;
                }
                let x1: usize = i - 1;
                let x2: usize = i + 1;
                let y1: usize = j - 1;
                let y2: usize = j + 1;
                let top_right = matrix[x1][y2];
                let bottom_right = matrix[x2][y2];
                let bottom_left = matrix[x2][y1];
                let top_left = matrix[x1][y1];
                if !((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M')){
                    continue;
                }
                if !((bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M')) {
                    continue;
                }
                sum += 1;
            }
        }
    }
    print!("{sum}")
}

fn count_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let mut sum = 0;
    for direction in Direction::iter() {
        if is_xmas(matrix, x, y, State::X(direction)) {
            println!("{}, {}", x, y);
            sum += 1;
        }
    }
    sum as u32
}

fn is_xmas(matrix: &Vec<Vec<char>>, x: usize, y: usize, state: State) -> bool {
    let height = matrix.len();
    let width = matrix[0].len();
    let i: i32;
    let j: i32;
    match state {
        State::DONE => return true,
        State::S => return matrix[x][y] == state.get_char_value().unwrap(),
        State::X(direction) | State::M(direction) | State::A(direction) => {
            if matrix[x][y] != state.get_char_value().unwrap() {
                return false;
            }
            (i, j) = direction.get_movement();
        }
    }
    let next_x = x as i32 + i;
    let next_y = y as i32 + j;
    if !is_within_bounds(matrix, next_x, next_y) {
        return false;
    }
    return is_xmas(
        matrix,
        next_x as usize,
        next_y as usize,
        state.get_next_state().unwrap(),
    );
}

fn is_within_bounds(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    let height = matrix.len();
    let width = matrix[0].len();
    if x < 0 || x >= height as i32 {
        return false;
    }
    if y < 0 || y >= width as i32 {
        return false;
    }
    return true;
}

#[derive(Clone, Copy, PartialEq, EnumIter)]
enum Direction {
    UP,
    UP_RIGHT,
    RIGHT,
    DOWN_RIGHT,
    DOWN,
    DOWN_LEFT,
    LEFT,
    UP_LEFT,
}

impl Direction {
    fn get_movement(&self) -> (i32, i32) {
        match self {
            Direction::UP => (-1, 0),
            Direction::UP_RIGHT => (-1, 1),
            Direction::RIGHT => (0, 1),
            Direction::DOWN_RIGHT => (1, 1),
            Direction::DOWN => (1, 0),
            Direction::DOWN_LEFT => (1, -1),
            Direction::LEFT => (0, -1),
            Direction::UP_LEFT => (-1, -1),
        }
    }
}

#[derive(PartialEq)]
enum State {
    X(Direction),
    M(Direction),
    A(Direction),
    S,
    DONE,
}

impl State {
    fn get_next_state(&self) -> Option<State> {
        match self {
            State::X(dir) => Some(State::M(*dir)),
            State::M(dir) => Some(State::A(*dir)),
            State::A(_) => Some(State::S),
            State::S => Some(State::DONE),
            State::DONE => None,
        }
    }

    fn get_direction(&self) -> Direction {
        match self {
            State::X(direction) => direction.to_owned(),
            State::M(direction) => direction.to_owned(),
            State::A(direction) => direction.to_owned(),
            State::S => panic!(),
            State::DONE => panic!(),
        }
    }

    fn get_char_value(&self) -> Option<char> {
        match self {
            State::X(_) => Some('X'),
            State::M(_) => Some('M'),
            State::A(_) => Some('A'),
            State::S => Some('S'),
            State::DONE => None,
        }
    }
}
