use std::{fs, ops::IndexMut};

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut trees: Vec<Vec<u8>> = Vec::new();
    for (i, line) in data.split("\n").enumerate() {
        trees.push(Vec::new());
        let tree_row = trees.index_mut(i);
        for char in line.chars() {
            tree_row.push(char.to_string().parse().unwrap());
        }
    }
    let width = trees[0].len();
    let height = trees.len();
    first_problem(height, width, &trees);
    let scenic_best = second_problem(trees, height, width);
    println!(
        "Scenic best with i: {}, j: {}, score: {}",
        scenic_best.0, scenic_best.1, scenic_best.2
    );
}

fn second_problem(trees: Vec<Vec<u8>>, height: usize, width: usize) -> (usize, usize, u32) {
    let mut scenic_best = (0, 0, get_scenic_score(0, 0, &trees));
    for i in 0..height {
        for j in 0..width {
            let scenic_score = get_scenic_score(i, j, &trees);
            if scenic_score > scenic_best.2 {
                scenic_best = (i, j, scenic_score);
            }
        }
    }
    scenic_best
}

fn first_problem(height: usize, width: usize, trees: &Vec<Vec<u8>>) {
    let mut visible_trees = 0;
    for i in 0..height {
        for j in 0..width {
            if is_tree_visible(i, j, &trees) {
                visible_trees += 1;
            }
        }
    }
    println!("{visible_trees}");
}

fn is_tree_visible(i: usize, j: usize, trees: &Vec<Vec<u8>>) -> bool {
    let width = trees[0].len();
    let height = trees.len();
    let tree_value = trees[i][j];
    if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
        return true;
    }
    let mut visible_left = true;
    for left in 0..j {
        if trees[i][left] >= tree_value.into() {
            visible_left = false;
            break;
        }
    }
    let mut visible_right = true;
    for right in j + 1..width {
        if trees[i][right] >= tree_value.into() {
            visible_right = false;
            break;
        }
    }
    let mut visible_top = true;
    for top in 0..i {
        if trees[top][j] >= tree_value.into() {
            visible_top = false;
            break;
        }
    }
    let mut visible_bottom = true;
    for bottom in i + 1..height {
        if trees[bottom][j] >= tree_value.into() {
            visible_bottom = false;
            break;
        }
    }
    println!("Tree with i: {}, j: {}", i, j);
    println!("Visible from left: {}", visible_left);
    println!("Visible from right: {}", visible_right);
    println!("Visible from top: {}", visible_top);
    println!("Visible from bottom: {}", visible_bottom);

    return visible_left || visible_right || visible_top || visible_bottom;
}

fn get_scenic_score(i: usize, j: usize, trees: &Vec<Vec<u8>>) -> u32 {
    let width = trees[0].len();
    let height = trees.len();
    let tree_value = trees[i][j];
    if i == 0 || i == height - 1 || j == 0 || j == width - 1 {
        return 0;
    }
    let mut scenic_left = 1;
    for left in (0..j).rev() {
        if trees[i][left] >= tree_value.into() {
            break;
        } else {
            if left == 0 {
                continue;
            }
            scenic_left += 1;
        }
    }
    let mut scenic_right = 1;
    for right in j + 1..width {
        if trees[i][right] >= tree_value.into() {
            break;
        } else {
            if right == width -1 {
                continue;
            }
            scenic_right += 1;
        }
    }
    let mut scenic_top = 1;
    for top in (0..i).rev() {
        if trees[top][j] >= tree_value.into() {
            break;
        } else {
            if top == 0  {
                continue;
            }
            scenic_top += 1;
        }
    }
    let mut scenic_bottom = 1;
    for bottom in i + 1..height {
        if trees[bottom][j] >= tree_value.into() {
            break;
        } else {
            if  bottom == height -1 {
                continue;
            }
            scenic_bottom += 1;
        }
    }
    println!("----Tree with i: {}, j: {}----", i, j);
    println!("Scenic left: {}", scenic_left);
    println!("Scenic right: {}", scenic_right);
    println!("Scenic top: {}", scenic_top);
    println!("Scenic bottom: {}", scenic_bottom);
    return scenic_left * scenic_right * scenic_top * scenic_bottom as u32;
}
