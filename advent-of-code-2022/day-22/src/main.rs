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

    

    let longest_line_length = board_data
        .split("\n")
        .into_iter()
        .map(|line| line.len())
        .max()
        .unwrap();

    let mut board: Vec<Vec<char>> = Vec::new();
    for (i, line) in board_data.split("\n").enumerate() {
        board.push(Vec::new());
        for j in 0..longest_line_length {
            if let Some(char) = line.chars().nth(j) {
                board[i].push(char);
            } else {
                board[i].push(' ');
            }
        }
    }
    let mut board_for_drawing = board.clone();
    let starting_position = get_starting_position(&board);
    println!("{:?}", &starting_position);
    let mut player = Player {
        x: starting_position.0,
        y: starting_position.1,
        direction: Direction::RIGHT,
    };
    get_next_tile(&board, &player);
    for instruction in str_strip_instructions(movement) {
        match instruction {
            "L" => player.direction = Player::get_next_direction(player.direction, Turn::LEFT),
            "R" => player.direction = Player::get_next_direction(player.direction, Turn::RIGHT),
            _ => {
                let steps = instruction.parse::<u32>().expect("Is a number");
                for _step in 0..steps {
                    let next_move = get_next_tile(&board, &player);
                    if player.x != next_move.0 || player.y != next_move.1 {
                        board_for_drawing[next_move.0][next_move.1] = match player.direction {
                            Direction::UP => '^',
                            Direction::RIGHT => '>',
                            Direction::DOWN => 'v',
                            Direction::LEFT => '<',
                        };
                    }
                    (player.x, player.y) = next_move;
                }
            }
        }
    }

    for ele in board_for_drawing {
        println!("{:?}", ele);
    }

    println!("Score: {}", get_score(&player));

    // println!("{}", data);
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn get_next_tile(board: &Vec<Vec<char>>, player: &Player) -> (usize, usize) {
    let potential_tile = match player.direction {
        Direction::UP => {
            if player.x == 0 || board[player.x - 1][player.y] == ' ' {
                let height = get_height(board);
                let mut tile = (0, 0);
                for i in (0..height).rev() {
                    if board[i][player.y] != ' ' {
                        tile = (i, player.y);
                        break;
                    }
                }
                tile
            } else {
                (player.x - 1, player.y)
            }
        }
        Direction::RIGHT => {
            let width = get_width(board);
            if player.y >= width - 1 || board[player.x][player.y + 1] == ' ' {
                let mut tile = (0, 0);
                for i in 0..width {
                    if board[player.x][i] != ' ' {
                        tile = (player.x, i);
                        break;
                    }
                }
                tile
            } else {
                (player.x, player.y + 1)
            }
        }
        Direction::DOWN => {
            let height = get_height(board);
            if player.x >= height - 1 || board[player.x + 1][player.y] == ' ' {
                let mut tile = (0, 0);
                for i in 0..height {
                    if board[i][player.y] != ' ' {
                        tile = (i, player.y);
                        break;
                    }
                }
                tile
            } else {
                (player.x + 1, player.y)
            }
        }
        Direction::LEFT => {
            let width = get_width(board);
            if player.y == 0 || board[player.x][player.y - 1] == ' ' {
                let mut tile = (0, 0);
                for i in (0..width).rev() {
                    if board[player.x][i] != ' ' {
                        tile = (player.x, i);
                        break;
                    }
                }
                tile
            } else {
                (player.x, player.y - 1)
            }
        }
    };
    if board[potential_tile.0][potential_tile.1] == '#' {
        return (player.x, player.y);
    }
    return potential_tile;
}

fn get_height(board: &Vec<Vec<char>>) -> usize {
    board.len()
}
fn get_width(board: &Vec<Vec<char>>) -> usize {
    board[0].len()
}

fn get_score(player: &Player) -> u32 {
    1000 * (player.x as u32 + 1) + 4 * (player.y as u32 + 1) + match player.direction {
        Direction::UP => 3,
        Direction::RIGHT => 0,
        Direction::DOWN => 1,
        Direction::LEFT => 2,
    }
}

//asume it's in the first row
fn get_starting_position(board: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..board.len() {
        if board[0][i] == '.' {
            return (0, i);
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


