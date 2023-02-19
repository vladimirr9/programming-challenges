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
    let mut visible_trees = 0;
    let width = trees[0].len();
    let height = trees.len();
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
    let mut visible_left  = true;
    for left in 0..j {
        if trees[i][left] >= tree_value.into()  {
            visible_left = false;
            break; 
        }
    }
    let mut visible_right  = true;
    for right in j+1..width {
        if trees[i][right] >= tree_value.into()  {
            visible_right = false;
            break; 
        }
    }
    let mut visible_top  = true;
    for top in 0..i {
        if trees[top][j] >= tree_value.into()  {
            visible_top = false;
            break; 
        }
    }
    let mut visible_bottom  = true;
    for bottom in i+1..height {
        if trees[bottom][j] >= tree_value.into()  {
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
