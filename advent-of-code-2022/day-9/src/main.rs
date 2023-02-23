use std::{collections::HashSet, fs};

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    
    first_problem(data);


    second_problem(data);

}

fn second_problem(data: &str) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0,0); 10];
    visited.insert((0,0));
    for line in data.split("\n") {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps: u32 = steps.parse().unwrap();
        for _step in 0..steps {
            let head = &mut rope[0];
            *head = match direction {
                "U" => (head.0, head.1 + 1),
                "R" => (head.0 + 1, head.1),
                "D" => (head.0, head.1 - 1),
                _ => (head.0 - 1, head.1),
            };
            for i in 0..9 {
                let leading_node = rope[i]; 
                let following_node = &mut rope[i+1]; 
                if is_adjacent(leading_node, *following_node) {
                    continue;
                }
                if following_node.0 == leading_node.0 {
                    if leading_node.1 > following_node.1 {
                        following_node.1 += 1;
                    } else {
                        following_node.1 -= 1;
                    }
                } else if following_node.1 == leading_node.1 {
                    if leading_node.0 > following_node.0 {
                        following_node.0 += 1;
                    } else {
                        following_node.0 -= 1;
                    }
                } else {
                    if leading_node.1 > following_node.1 {
                        following_node.1 += 1;
                    } else {
                        following_node.1 -= 1;
                    }
                    if leading_node.0 > following_node.0 {
                        following_node.0 += 1;
                    } else {
                        following_node.0 -= 1;
                    }
                }
            }
            let tail = rope[9];
            visited.insert(tail);
        }
    }
    println!("{}", visited.len())
}

fn first_problem(data: &str) {
    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    for line in data.split("\n") {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps: u32 = steps.parse().unwrap();
        for _step in 0..steps {
            head = match direction {
                "U" => (head.0, head.1 + 1),
                "R" => (head.0 + 1, head.1),
                "D" => (head.0, head.1 - 1),
                _ => (head.0 - 1, head.1),
            };
            if is_adjacent(head, tail) {
                continue;
            }
            if tail.0 == head.0 {
                if head.1 > tail.1 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
            } else if tail.1 == head.1 {
                if head.0 > tail.0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            } else {
                if head.1 > tail.1 {
                    tail.1 += 1;
                } else {
                    tail.1 -= 1;
                }
                if head.0 > tail.0 {
                    tail.0 += 1;
                } else {
                    tail.0 -= 1;
                }
            }
            println!("({},{})", tail.0, tail.1);
            visited.insert(tail);
        }
    }
    println!("{}", visited.len())
}

fn is_adjacent(node_1: (i32, i32), node_2: (i32, i32)) -> bool {
    for i in -1..2 {
        for j in -1..2 {
            if (node_1.0 + i, node_1.1 + j) == node_2 {
                return true;
            }
        }
    }
    return false;
}
