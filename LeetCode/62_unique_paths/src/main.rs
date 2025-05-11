use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}


pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut calculated_vals : HashMap<(i32,i32), i32> = HashMap::new();
    return unique_paths_internal(m-1, n-1, m, n, &mut calculated_vals);
}




pub fn unique_paths_internal(i: i32, j: i32, m: i32, n: i32, calculated_vals: &mut HashMap<(i32,i32), i32>) -> i32 {
    if i == 0 && j == 0 {
        return 1;
    }
    let left = j - 1;
    let left_side_unique_paths = if left < 0{
        0
    } else {
        if calculated_vals.contains_key(&(i, left)) {
            *calculated_vals.get(&(i,left)).unwrap()
        } else {
            unique_paths_internal(i, left, m, n, calculated_vals)
        }
    };
    let top = i - 1;
    let top_side_unique_paths = if top < 0 {
        0
    } else {
        if calculated_vals.contains_key(&(top, j)) {
            *calculated_vals.get(&(top,j)).unwrap()
        } else {
            unique_paths_internal(top, j, m, n, calculated_vals)
        }
    };
    let field_val = left_side_unique_paths + top_side_unique_paths;
    calculated_vals.insert((i,j), field_val);
    return field_val;
}


#[test]
fn first_test() {
    assert_eq!(unique_paths(3, 7), 28)
}

#[test]
fn second_test() {
    assert_eq!(unique_paths(3, 2), 3)
}