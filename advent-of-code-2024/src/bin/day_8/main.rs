use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::parsing_utils;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let content = parsing_utils::get_contents(8, false, 1).trim().to_owned();
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let height = matrix.len();
    let width = matrix[0].len();

    for i in 0..height {
        for j in 0..width {
            let char = matrix[i][j];
            if char.is_ascii_alphanumeric() {
                if antennas.get(&char).is_none() {
                    antennas.insert(char, Vec::new());
                }
                antennas.get_mut(&char).unwrap().push((i, j));
            }
        }
    }
    let mut unique_antinode_fields: HashSet<(usize, usize)> = HashSet::new();
    for antenna_locations in antennas.values() {
        if antenna_locations.len() <= 1 {
            continue;
        }
        for i in 0..antenna_locations.len() {
            for j in i + 1..antenna_locations.len() {
                let a = antenna_locations[i];
                let b = antenna_locations[j];
                let (x1, y1) = a;
                let (x2, y2) = b;
                let x_abs = x1.abs_diff(x2) as i32;
                let y_abs = y1.abs_diff(y2) as i32;
                let antinode_a_x_direction = if x_abs == 0 {
                    0
                } else {
                    (x1 as i32 - x2 as i32) / x_abs
                };
                let antinode_a_y_direction = if y_abs == 0 {
                    0
                } else {
                    (y1 as i32 - y2 as i32) / y_abs
                };

                let antinode_b_x_direction = if x_abs == 0 {
                    0
                } else {
                    (x2 as i32 - x1 as i32) / x_abs
                };
                let antinode_b_y_direction = if y_abs == 0 {
                    0
                } else {
                    (y2 as i32 - y1 as i32) / y_abs
                };

                let antinode_a = (
                    x1 as i32 + antinode_a_x_direction * x_abs,
                    y1 as i32 + antinode_a_y_direction * y_abs,
                );
                let antinode_b = (
                    x2 as i32 + antinode_b_x_direction * x_abs,
                    y2 as i32 + antinode_b_y_direction * y_abs,
                );
                println!("{:?}, {:?}", antinode_a, antinode_b);

                let (anti_x1, anti_y1) = antinode_a;
                if (anti_x1 >= 0 && anti_x1 < height as i32)
                    && (anti_y1 >= 0 && anti_y1 < width as i32)
                {
                    unique_antinode_fields.insert((anti_x1 as usize, anti_y1 as usize));
                }
                let (anti_x2, anti_y2) = antinode_b;
                if (anti_x2 >= 0 && anti_x2 < height as i32)
                    && (anti_y2 >= 0 && anti_y2 < width as i32)
                {
                    unique_antinode_fields.insert((anti_x2 as usize, anti_y2 as usize));
                }
            }
        }
    }
    // print_map(&matrix, &unique_antinode_fields);
    println!("{}", unique_antinode_fields.len());
}

fn second_problem() {
    let content = parsing_utils::get_contents(8, false, 1).trim().to_owned();
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let height = matrix.len();
    let width = matrix[0].len();

    for i in 0..height {
        for j in 0..width {
            let char = matrix[i][j];
            if char.is_ascii_alphanumeric() {
                if antennas.get(&char).is_none() {
                    antennas.insert(char, Vec::new());
                }
                antennas.get_mut(&char).unwrap().push((i, j));
            }
        }
    }
    let mut unique_antinode_fields: HashSet<(usize, usize)> = HashSet::new();
    for antenna_locations in antennas.values() {
        if antenna_locations.len() <= 1 {
            continue;
        }
        for i in 0..antenna_locations.len() {
            for j in i + 1..antenna_locations.len() {
                let a = antenna_locations[i];
                let b = antenna_locations[j];
                let (x1, y1) = a;
                let (x2, y2) = b;
                let x_abs = x1.abs_diff(x2) as i32;
                let y_abs = y1.abs_diff(y2) as i32;
                let antinode_a_x_direction = if x_abs == 0 {
                    0
                } else {
                    (x1 as i32 - x2 as i32) / x_abs
                };
                let antinode_a_y_direction = if y_abs == 0 {
                    0
                } else {
                    (y1 as i32 - y2 as i32) / y_abs
                };

                let antinode_b_x_direction = if x_abs == 0 {
                    0
                } else {
                    (x2 as i32 - x1 as i32) / x_abs
                };
                let antinode_b_y_direction = if y_abs == 0 {
                    0
                } else {
                    (y2 as i32 - y1 as i32) / y_abs
                };
                for i in 0.. {
                    let antinode_a = (
                        x1 as i32 + antinode_a_x_direction * i * x_abs,
                        y1 as i32 + antinode_a_y_direction * i * y_abs,
                    );
                    let antinode_b = (
                        x2 as i32 + antinode_b_x_direction * i * x_abs,
                        y2 as i32 + antinode_b_y_direction * i * y_abs,
                    );
                    let (anti_x1, anti_y1) = antinode_a;
                    let (anti_x2, anti_y2) = antinode_b;
                    if ((anti_x1 < 0 || anti_x1 >= height as i32)
                        || (anti_y1 < 0 || anti_y1 >= width as i32))
                        && ((anti_x2 < 0 || anti_x2 >= height as i32)
                            || (anti_y2 < 0 || anti_y2 >= width as i32))
                    {
                        break;
                    }
                    if (anti_x1 >= 0 && anti_x1 < height as i32)
                        && (anti_y1 >= 0 && anti_y1 < width as i32)
                    {
                        unique_antinode_fields.insert((anti_x1 as usize, anti_y1 as usize));
                    }

                    if (anti_x2 >= 0 && anti_x2 < height as i32)
                        && (anti_y2 >= 0 && anti_y2 < width as i32)
                    {
                        unique_antinode_fields.insert((anti_x2 as usize, anti_y2 as usize));
                    }
                }

                let antinode_a = (
                    x1 as i32 + antinode_a_x_direction * x_abs,
                    y1 as i32 + antinode_a_y_direction * y_abs,
                );
                let antinode_b = (
                    x2 as i32 + antinode_b_x_direction * x_abs,
                    y2 as i32 + antinode_b_y_direction * y_abs,
                );
                println!("{:?}, {:?}", antinode_a, antinode_b);

                let (anti_x1, anti_y1) = antinode_a;
                if (anti_x1 >= 0 && anti_x1 < height as i32)
                    && (anti_y1 >= 0 && anti_y1 < width as i32)
                {
                    unique_antinode_fields.insert((anti_x1 as usize, anti_y1 as usize));
                }
                let (anti_x2, anti_y2) = antinode_b;
                if (anti_x2 >= 0 && anti_x2 < height as i32)
                    && (anti_y2 >= 0 && anti_y2 < width as i32)
                {
                    unique_antinode_fields.insert((anti_x2 as usize, anti_y2 as usize));
                }
            }
        }
    }
    print_map(&matrix, &unique_antinode_fields);
    println!("{}", unique_antinode_fields.len());
}

fn print_map(matrix: &Vec<Vec<char>>, unique_antinode_fields: &HashSet<(usize, usize)>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            print!("{}", matrix[i][j]);
        }
        println!();
    }
    println!("=======================");
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if unique_antinode_fields.get(&(i, j)).is_some() {
                print!("#");
            } else {
                print!("{}", matrix[i][j]);
            }
        }
        println!();
    }
}
