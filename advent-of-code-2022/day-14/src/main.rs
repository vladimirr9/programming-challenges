use std::{
    cmp::{max, min},
    fs,
};
// NOTE:
// Y indicates column
// X indicates row
// In the text of the exercise it's the opposite

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let width = 1000;
    let height = 300;
    let mut slice = vec![vec!['.'; width]; height];
    let sand = (0, 500);
    let mut sand_count = 0;
    slice[sand.0][sand.1] = '+';
    for path in data.split("\n") {
        let coords: Vec<&str> = path.split("->").map(|coord| coord.trim()).collect();
        for i in 1..coords.len() {
            let (y1, x1) = coords[i - 1].split_once(",").unwrap();
            let (y1, x1) = (y1.parse::<usize>().unwrap(), x1.parse::<usize>().unwrap());
            let (y2, x2) = coords[i].split_once(",").unwrap();
            let (y2, x2) = (y2.parse::<usize>().unwrap(), x2.parse::<usize>().unwrap());

            let (x1, x2) = (min(x1, x2), max(x1, x2));
            let (y1, y2) = (min(y1, y2), max(y1, y2));

            for x in x1..=x2 {
                slice[x][y1] = '#';
            }
            for y in y1..=y2 {
                slice[x1][y] = '#';
            }
        }
    }
    'outer: loop {
        let mut position = (sand.0 + 1, sand.1);
        loop {
            let next_position = get_sand_move(&slice, position.0, position.1);
            match next_position {
                None => {
                    slice[position.0][position.1] = 'o';
                    sand_count += 1;
                    continue 'outer;
                }
                Some(i) => {
                    position = i;
                    if position.0 == height-1 {
                        break 'outer;
                    }
                }
            };
        }
    }
    print_part_slice(&slice, 0, 10, 493, 505);
    println!("Sand count {}", sand_count);
}

fn get_sand_move(slice: &Vec<Vec<char>>, x1: usize, y1: usize) -> Option<(usize, usize)> {
    if slice[x1 + 1][y1] == '.' {
        return Some((x1 + 1, y1));
    }
    if slice[x1 + 1][y1 - 1] == '.' {
        return Some((x1 + 1, y1 - 1));
    }
    if slice[x1 + 1][y1 + 1] == '.' {
        return Some((x1 + 1, y1 + 1));
    }
    None
}

fn print_part_slice(slice: &Vec<Vec<char>>, x1: usize, x2: usize, y1: usize, y2: usize) {
    for i in x1..x2 {
        for j in y1..y2 {
            print!("{}", slice[i][j]);
        }
        println!();
    }
    println!("=========================");
}
