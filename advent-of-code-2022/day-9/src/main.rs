use std::{collections::HashSet, fs};

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
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
            if is_tail_adjacent(head, tail) {
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

fn is_tail_adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
    for i in -1..2 {
        for j in -1..2 {
            if (head.0 + i, head.1 + j) == tail {
                return true;
            }
        }
    }
    return false;
}
