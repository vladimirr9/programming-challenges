use std::{fs, cell::Cell};

struct JetPatternStream {
    jet_pattern: String,
    pointer: Cell<usize>,
}
#[derive(Debug)]
enum JetPattern {
    LEFT,
    RIGHT
}

impl JetPatternStream {
    fn get_jet_pattern(&mut self) -> JetPattern {
        let val = Cell::new(self.jet_pattern.chars().nth(self.pointer.get()).unwrap());
        self.pointer = Cell::new((self.pointer.get() + 1) % self.jet_pattern.len());
        if val.get() == '<' {
            return JetPattern::LEFT;
        } else {
            return JetPattern::RIGHT;
        }
    }
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut jet_pattern = JetPatternStream {
        jet_pattern: String::from(data),
        pointer: Cell::new(0)
    };

    for i in 0..50 {
        println!("{:?}", jet_pattern.get_jet_pattern());
    }   
}
