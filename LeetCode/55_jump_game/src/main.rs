use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}


pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let mut can_jump = false;
    let mut visited_indices: HashSet<i32> = HashSet::new();
    visited_indices.insert(0);
    let mut indices_to_look_at = get_potential_target_indices(0, &nums);

    while !indices_to_look_at.is_empty() {
        let some_index = indices_to_look_at.pop().unwrap();
        if some_index == (nums.len() - 1) as i32 {
            can_jump = true;
            break;
        }
        if visited_indices.contains(&some_index) {
            continue;
        }
        visited_indices.insert(some_index);

        let mut more_indices_to_look_at = get_potential_target_indices(some_index, &nums);
        more_indices_to_look_at.retain(|val| !visited_indices.contains(val));
        indices_to_look_at.append(&mut more_indices_to_look_at);
    }

    return can_jump;
}

pub fn get_potential_target_indices(index: i32, nums: &Vec<i32>) -> Vec<i32> {
    let value = nums[index as usize];
    let mut indices: Vec<i32> = Vec::new();
    for i in 1..value+1 {
        let potential_index = index + i;
        if potential_index >= 0 && potential_index < nums.len() as i32 {
            indices.push(potential_index);
        }
        let potential_index = index - i;
        if potential_index >= 0 && potential_index < nums.len() as i32 {
            indices.push(potential_index);
        }
    }
    println!("idx: {}, nums: {:?}", index, &indices);
    return indices;

}

#[test]
fn first_test() {
    assert!(can_jump(vec![2,3,1,1,4]))
}


#[test]
fn second_test() {
    assert!(!can_jump(vec![3,2,1,0,4]))
}