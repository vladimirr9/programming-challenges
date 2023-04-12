use std::{fs, time::Instant};

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}


struct Player {
    x: usize,
    y: usize,
    direction: Direction
}


fn main() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim_end();

    let (board_data, movement) = data.split_once("\n\n").unwrap();


    let mut board: Vec<Vec<char>> =  Vec::new();
    for (i, line) in board_data.split("\n").enumerate() {
        board.push(Vec::new());
        for char in line.chars() {
            board[i].push(char);
        }
    }
    let starting_position = get_starting_position(&board);
    println!("{:?}", &starting_position);
    let mut player = Player{
        x: starting_position.0,
        y: starting_position.1,
        direction: Direction::RIGHT,
    };

    

    println!("{:?}", board);

    // println!("{}", data);
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}


//asume it's in the first column
fn get_starting_position(board :&Vec<Vec<char>>) -> (usize, usize) {
    let mut position: (usize,usize) = (0,0);
    for i in 0..board.len() {
        if board[i][0] == '.' {
            return (i, 0);
        }
    }
    panic!();
}