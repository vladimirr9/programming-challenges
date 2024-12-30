use std::collections::HashSet;

use advent_of_code_2024::utils::parsing_utils;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let content = parsing_utils::get_contents(10, false, 1);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if matrix[i][j] == '0' {
                sum += get_tops(&matrix, i, j).len()
            }
        }
    }
    println!("{}", sum);
}



fn second_problem() {
    let content = parsing_utils::get_contents(10, false, 1);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if matrix[i][j] == '0' {
                sum += get_score(&matrix, i, j)
            }
        }
    }
    println!("{}", sum);
}


fn get_score(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if matrix[i][j] == '9' {
        return 1;
    }
    let mut sum = 0;
    let value = matrix[i][j].to_string().parse::<u32>().unwrap();
    for valid_neighbour in get_valid_neighbours(matrix, i, j) {
        let value_of_neighbour = matrix[valid_neighbour.0][valid_neighbour.1];
        if !value_of_neighbour.is_ascii_digit() {
            continue;
        }
        let value_of_neighbour = value_of_neighbour.to_string().parse::<u32>().unwrap();
        if value_of_neighbour > value && value_of_neighbour.abs_diff(value) == 1 {
            let neighbor_tops = get_score(matrix, valid_neighbour.0, valid_neighbour.1);
            sum += get_score(matrix, valid_neighbour.0, valid_neighbour.1);
        }
    }
    return sum
}



fn get_tops(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<(usize, usize)> {
    let mut tops: HashSet<(usize, usize)> = HashSet::new();
    if matrix[i][j] == '9' {
        tops.insert((i,j));
        return tops;
    }
    let value = matrix[i][j].to_string().parse::<u32>().unwrap();
    for valid_neighbour in get_valid_neighbours(matrix, i, j) {
        let value_of_neighbour = matrix[valid_neighbour.0][valid_neighbour.1];
        if !value_of_neighbour.is_ascii_digit() {
            continue;
        }
        let value_of_neighbour = value_of_neighbour.to_string().parse::<u32>().unwrap();
        if value_of_neighbour > value && value_of_neighbour.abs_diff(value) == 1 {
            let neighbor_tops = get_tops(matrix, valid_neighbour.0, valid_neighbour.1);
            tops = tops.union(&neighbor_tops).map(|val| val.clone()).collect();
        }
    }
    return tops
}

fn get_valid_neighbours(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for (x_change, y_change) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let potential_x = i as i32 + x_change;
        let potential_y = j as i32 + y_change;
        if potential_x >= 0
            && potential_x < height as i32
            && potential_y >= 0
            && potential_y < width as i32
        {
            neighbours.push((potential_x as usize, potential_y as usize));
        }
    }
    return neighbours;
}
