use std::{cell::Cell, fs, iter::zip, time::Instant};

// X ^
//   |
//   |
//   |
//   |
//   +------------>
// (0,0)          Y
//
//
//

struct JetPatternStream {
    jet_pattern: String,
    pointer: Cell<usize>,
}
#[derive(Debug, Clone, Copy)]
enum JetPattern {
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug)]
enum RockType {
    H_BAR,
    PLUS,
    CORNER,
    V_BAR,
    BOX,
}

struct RockTypeStream {
    rock_pattern: Vec<RockType>,
    pointer: Cell<usize>,
}

impl RockTypeStream {
    fn get_rock_type(&mut self) -> RockType {
        let val = Cell::new(self.rock_pattern[self.pointer.get()]);
        self.pointer = Cell::new((self.pointer.get() + 1) % self.rock_pattern.len());
        return val.get().clone();
    }
}
// (x,y) is the leftmost point
struct Rock {
    x: usize,
    y: usize,
    points: Vec<(usize, usize)>,
}

impl Rock {
    fn get_rock_points(x: usize, y: usize, rock_type: RockType) -> Vec<(usize, usize)> {
        let mut points = vec![(x, y)];
        match rock_type {
            RockType::H_BAR => {
                points.push((x, y + 1));
                points.push((x, y + 2));
                points.push((x, y + 3));
            }
            RockType::PLUS => {
                points.push((x, y + 1));
                points.push((x, y + 2));
                points.push((x + 1, y + 1));
                points.push((x - 1, y + 1));
            }
            RockType::CORNER => {
                points.push((x, y + 1));
                points.push((x, y + 2));
                points.push((x + 1, y + 2));
                points.push((x + 2, y + 2));
            }
            RockType::V_BAR => {
                points.push((x + 1, y));
                points.push((x + 2, y));
                points.push((x + 3, y));
            }
            RockType::BOX => {
                points.push((x, y + 1));
                points.push((x + 1, y));
                points.push((x + 1, y + 1));
            }
        };
        points
    }

    fn get_distance_leftmost_lowest(rock_type: RockType) -> usize {
        match rock_type {
            RockType::H_BAR => 0,
            RockType::PLUS => 1,
            RockType::CORNER => 0,
            RockType::V_BAR => 0,
            RockType::BOX => 0,
        }
    }
}

fn main() {
    
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let now = Instant::now();

    let mut jet_pattern: Vec<JetPattern> = Vec::new();
    let mut jet_pointer: usize = 0;

    for char in data.chars() {
        if char == '<' {
            jet_pattern.push(JetPattern::LEFT)
        } else {
            jet_pattern.push(JetPattern::RIGHT)
        }
    }
    let mut rock_pointer: usize = 0;
    let rock_pattern = vec![
        RockType::H_BAR,
        RockType::PLUS,
        RockType::CORNER,
        RockType::V_BAR,
        RockType::BOX,
    ];

    let mut board: Vec<u8> = vec![0b1000_0000; 3_000_000_000];
    // let mut board: Vec<u8> = vec![0b1000_0000, 0b1000_0001, 0b1000_0010, 0b1000_0100, 0b1000_1000, 0b1001_0000, 0b1010_0000, 0b1100_0000, 0b1000_0000];
    // let num_of_rocks: u64 = 1_000_000_000_000;
    // let num_of_rocks: u64 = 2022;
    let num_of_rocks: u64 = 1_000_000_000;
    let mut highest_point: usize = 0;

    // print_board(&board);

    for i in 0..num_of_rocks {
        let rock_type = get_rock_type(&rock_pattern, &mut rock_pointer);
        // println!("{}", highest_point);
        let mut x = if i == 0 { 3 } else { highest_point + 4 };
        let mut rock_bytes = get_rock_bytes(&rock_type);
        loop {
            let jet_direction = get_jet_pattern(&jet_pattern, &mut jet_pointer);
            let board_rows = &board[x..x + rock_bytes.len()];
            match jet_direction {
                JetPattern::LEFT => {
                    if zip(&rock_bytes, board_rows).all(|(rock_byte, row)| {
                        rock_byte & 0b1100_0000 == 0 && rock_byte << 1 & row == 0
                    }) {
                        // println!("LEFT {:?}", rock_type);
                        for i in 0..rock_bytes.len() {
                            rock_bytes[i] = rock_bytes[i] << 1;
                        }
                    }
                }
                JetPattern::RIGHT => {
                    if zip(&rock_bytes, board_rows).all(|(rock_byte, row)| {
                        rock_byte & 0b1000_0001 == 0 && rock_byte >> 1 & row == 0
                    }) {
                        // println!("RIGHT {:?}", rock_type);
                        for i in 0..rock_bytes.len() {
                            rock_bytes[i] = rock_bytes[i] >> 1;
                        }
                    }
                }
            }
            if x > 0 {
                let new_x = x - 1;
                let board_rows = &board[new_x..new_x + rock_bytes.len()];
                if zip(&rock_bytes, board_rows).all(|(rock_byte, row)| rock_byte & row == 0) {
                    x = x - 1;
                } else {
                    for i in 0..rock_bytes.len() {
                        board[x + i] = board[x + i] | rock_bytes[i];
                    }
                    if x + rock_bytes.len() > highest_point {
                        highest_point = x + rock_bytes.len() - 1;
                    }
                    break;
                }
            } else {
                for i in 0..rock_bytes.len() {
                    board[x + i] = board[x + i] | rock_bytes[i];
                }
                if x + rock_bytes.len() > highest_point {
                    highest_point = x + rock_bytes.len() - 1;
                }
                break;
            }
        }
        // print_board(&board)
    }
    println!("{}", highest_point + 1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn get_rock_bytes(rock_type: &RockType) -> Vec<u8> {
    match rock_type {
        RockType::H_BAR => vec![0b0001_1110],
        RockType::PLUS => vec![0b0000_1000, 0b0001_1100, 0b0000_1000],
        RockType::CORNER => vec![0b0001_1100, 0b0000_0100, 0b0000_0100],
        RockType::V_BAR => vec![0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000],
        RockType::BOX => vec![0b0001_1000, 0b0001_1000],
    }
}

fn get_jet_pattern(jet_pattern: &Vec<JetPattern>, pointer: &mut usize) -> JetPattern {
    let val = jet_pattern[*pointer];
    *pointer += 1;
    if *pointer >= jet_pattern.len() {
        *pointer = 0;
    }
    return val;
}

fn get_rock_type(rock_pattern: &Vec<RockType>, pointer: &mut usize) -> RockType {
    let val = rock_pattern[*pointer];
    *pointer += 1;
    if *pointer >= rock_pattern.len() {
        *pointer = 0;
    }
    return val;
}

fn print_board(board: &Vec<u8>) {
    println!("=====================");
    for i in (0..8).rev() {
        let binary_str = format!("{:08b}", board[i]);
        println!("{}", binary_str);
    }
    println!("=====================");
}

fn print_rock(rock: &Vec<u8>) {
    println!("=====================");
    for i in (0..rock.len()).rev() {
        let binary_str = format!("{:08b}", rock[i]);
        println!("{}", binary_str);
    }
    println!("=====================");
}