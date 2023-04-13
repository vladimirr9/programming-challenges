use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, time::Instant};
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

enum Turn {
    LEFT,
    RIGHT,
}

impl Player {
    fn get_next_direction(direction: Direction, turn: Turn) -> Direction {
        match turn {
            Turn::LEFT => match direction {
                Direction::UP => Direction::LEFT,
                Direction::RIGHT => Direction::UP,
                Direction::DOWN => Direction::RIGHT,
                Direction::LEFT => Direction::DOWN,
            },
            Turn::RIGHT => match direction {
                Direction::UP => Direction::RIGHT,
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
            },
        }
    }
}

struct Player {
    x: usize,
    y: usize,
    direction: Direction,
}

fn main() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim_end();

    let (board_data, movement) = data.split_once("\n\n").unwrap();

    let mut board: Vec<Vec<char>> = Vec::new();
    for (i, line) in board_data.split("\n").enumerate() {
        board.push(Vec::new());
        for char in line.chars() {
            board[i].push(char);
        }
    }
    let starting_position = get_starting_position(&board);
    println!("{:?}", &starting_position);
    let mut player = Player {
        x: starting_position.0,
        y: starting_position.1,
        direction: Direction::RIGHT,
    };

    for instruction in str_strip_instructions(movement) {
        match instruction {
            "L" => player.direction = Player::get_next_direction(player.direction, Turn::LEFT),
            "R" => player.direction = Player::get_next_direction(player.direction, Turn::RIGHT),
            _ => {
                let steps = instruction.parse::<u32>().expect("Is a number");
                for step in 0..steps {

                }
            }
        }
    }

    println!("{:?}", board);

    // println!("{}", data);
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}


fn get_next_tile(board: &Vec<Vec<char>>, player: &Player) -> (usize, usize) {
    let mut next_tile = (player.x, player.y);
    match player.direction {
        Direction::UP => if player.x == 0 || board[player.x - 1][player.y] != '.' {
            
        },
        Direction::RIGHT => todo!(),
        Direction::DOWN => todo!(),
        Direction::LEFT => todo!(),
    }
}


fn get_height(board: &Vec<Vec<char>>) -> usize {
    board.len()
}
fn get_width(board: &Vec<Vec<char>>, row: usize) -> usize {
    board[row].len()
}


//asume it's in the first column
fn get_starting_position(board: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..board.len() {
        if board[i][0] == '.' {
            return (i, 0);
        }
    }
    panic!();
}

fn str_strip_instructions(instructions: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([L]|[R]|[0-9]+)").unwrap();
    }
    return RE
        .find_iter(instructions)
        .map(|item| item.as_str())
        .collect();
}
