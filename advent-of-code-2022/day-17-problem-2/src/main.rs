use std::{fs, iter::zip, time::Instant};


#[derive(Debug, Clone, Copy)]
enum JetPattern {
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Debug)]
enum RockType {
    HBAR,
    PLUS,
    CORNER,
    VBAR,
    BOX,
}

// This took me like 10.5 hours to complete, but this is the first solution I came up with and it works.
// I got 1585673352422 as the result.
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
        RockType::HBAR,
        RockType::PLUS,
        RockType::CORNER,
        RockType::VBAR,
        RockType::BOX,
    ];

    let mut board: Vec<u8> = vec![0b1000_0000; 5_000_000_000];
    let num_of_rocks: u64 = 1_000_000_000_000;
    let cutoff : usize = 100_000_000;
    let mut highest_point: usize = 0;
    let mut total_cutoff : usize = 0;


    for i in 0..num_of_rocks {
        let rock_type = get_rock_type(&rock_pattern, &mut rock_pointer);
        let mut x = if i == 0 { 3 } else { highest_point + 4 };
        let mut rock_bytes = get_rock_bytes(&rock_type);
        if x / cutoff > 2 {
            total_cutoff += cutoff;
            highest_point -= cutoff;
            board.drain(0..cutoff);
            board.append(&mut vec![0b1000_0000; cutoff]);
            x = highest_point + 4;
        }
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
    }
    println!("{}", highest_point + total_cutoff + 1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn get_rock_bytes(rock_type: &RockType) -> Vec<u8> {
    match rock_type {
        RockType::HBAR => vec![0b0001_1110],
        RockType::PLUS => vec![0b0000_1000, 0b0001_1100, 0b0000_1000],
        RockType::CORNER => vec![0b0001_1100, 0b0000_0100, 0b0000_0100],
        RockType::VBAR => vec![0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000],
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

