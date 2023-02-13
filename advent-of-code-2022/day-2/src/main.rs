use std::fs;

fn main() {
    // first_problem()
    second_problem();
}

fn second_problem() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let rows: Vec<&str> = data.split("\n").collect();
    let mut total_score = 0;
    for row in rows {
        let (first, second) = row.split_once(" ").unwrap();
        total_score += match second {
            "X" => {
                0 + match first {
                    "A" => 3,
                    "B" => 1,
                    _ => 2,
                }
            }
            "Y" => {
                3 + match first {
                    "A" => 1,
                    "B" => 2,
                    _ => 3,
                }
            }
            _ => {
                6 + match first {
                    "A" => 2,
                    "B" => 3,
                    _ => 1,
                }
            }
        }
    }
    println!("{total_score}");
}




fn first_problem() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let rows: Vec<&str> = data.split("\n").collect();
    let mut total_score = 0;
    for row in rows {
        let (first, second) = row.split_once(" ").unwrap();
        total_score += match second {
            "X" => {
                1 + match first {
                    "A" => 3,
                    "B" => 0,
                    _ => 6,
                }
            }
            "Y" => {
                2 + match first {
                    "A" => 6,
                    "B" => 3,
                    _ => 0,
                }
            }
            _ => {
                3 + match first {
                    "A" => 0,
                    "B" => 6,
                    _ => 3,
                }
            }
        }
    }
    println!("{total_score}");
}
