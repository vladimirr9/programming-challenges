use std::{cell::Cell, fs};

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
#[derive(Debug)]
enum JetPattern {
    LEFT,
    RIGHT,
}

impl JetPatternStream {
    fn get_jet_pattern(&mut self) -> JetPattern {
        let val = Cell::new(self.jet_pattern.chars().nth(self.pointer.get()).unwrap());
        self.pointer = Cell::new((self.pointer.get() + 1) % self.jet_pattern.len());
        if val.get() == '<' {
            return JetPattern::LEFT;
        } else {
            return JetPattern::RIGHT;
        }
    }
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
    let mut jet_pattern = JetPatternStream {
        jet_pattern: String::from(data),
        pointer: Cell::new(0),
    };
    let mut rock_pattern = RockTypeStream {
        rock_pattern: vec![
            RockType::H_BAR,
            RockType::PLUS,
            RockType::CORNER,
            RockType::V_BAR,
            RockType::BOX,
        ],
        pointer: Cell::new(0),
    };
    
    let board_width = 7;
    let mut board = vec![vec!['.'; board_width]; 10000];
    let num_of_rocks = 2022;
    let mut highest_point = 0;
    for i in 0..num_of_rocks {
        let rock_type = rock_pattern.get_rock_type();
        let x = if i == 0 {
            3 + Rock::get_distance_leftmost_lowest(rock_type)
        } else {
            highest_point + 4 + Rock::get_distance_leftmost_lowest(rock_type)
        };
        let mut rock = Rock {
            x: x,
            y: 2,
            points: Rock::get_rock_points(x, 2, rock_type),
        };
        loop {
            let jet_direction = jet_pattern.get_jet_pattern();
            match jet_direction {
                JetPattern::LEFT => {
                    if rock
                        .points
                        .iter()
                        .all(|point| point.1 > 0 && board[point.0][point.1 - 1] == '.')
                    {
                        rock.y -= 1;
                        for i in 0..rock.points.len() {
                            rock.points[i].1 -= 1;
                        }
                    }
                }
                JetPattern::RIGHT => {
                    if rock.points.iter().all(|point| {
                        point.1 < board_width - 1 && board[point.0][point.1 + 1] == '.'
                    }) {
                        rock.y += 1;
                        for i in 0..rock.points.len() {
                            rock.points[i].1 += 1;
                        }
                    }
                }
            }
            if rock
                .points
                .iter()
                .all(|point| point.0 > 0 && board[point.0 - 1][point.1] == '.')
            {
                rock.x -= 1;
                for i in 0..rock.points.len() {
                    rock.points[i].0 -= 1;
                }
                
            } else {
                for point in &rock.points {
                    board[point.0][point.1] = '#';
                    if point.0 >= highest_point {
                        highest_point = point.0;
                    }
                }
                break;
            }
        }
    }
    
    println!("{}", highest_point + 1);
}


fn print_board(board: &Vec<Vec<char>>) {
    println!("=====================");
    for i in (0..12).rev() {
        println!("{:?}", board[i]);
    }
    println!("=====================");
}