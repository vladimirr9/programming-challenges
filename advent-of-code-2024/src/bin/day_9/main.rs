use advent_of_code_2024::utils::parsing_utils;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let content = parsing_utils::get_contents(9, false, 1).trim().to_owned();
    let mut filesystem: Vec<Block> = Vec::new();
    let mut id: u64 = 0;
    let mut is_file = true;
    let mut sum = 0;
    for char in content.chars() {
        let value = char.to_string().parse::<u64>().unwrap();
        for _i in 0..value {
            if is_file {
                filesystem.push(Block::File(id));
            } else {
                filesystem.push(Block::Empty);
            }
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    let mut p = 0;
    let mut q = filesystem.len() - 1;
    while p <= q {
        if filesystem[p] != Block::Empty {
            p += 1;
            continue;
        }
        if let Block::Empty = filesystem[q] {
            q -= 1;
            continue;
        }
        filesystem[p] = filesystem[q].clone();
        filesystem[q] = Block::Empty;
    }
    for i in 0..filesystem.len() {
        match filesystem[i] {
            Block::Empty => break,
            Block::File(id) => sum += i as u64 * id,
        }
    }
    println!("{:?}", filesystem);
    println!("{:?}", sum);
}

fn second_problem() {
    let content = parsing_utils::get_contents(9, false, 1).trim().to_owned();
    let mut filesystem: Vec<Block> = Vec::new();
    let mut id: u64 = 0;
    let mut is_file = true;
    let mut sum = 0;
    for char in content.chars() {
        let value = char.to_string().parse::<u64>().unwrap();
        for _i in 0..value {
            if is_file {
                filesystem.push(Block::File(id));
            } else {
                filesystem.push(Block::Empty);
            }
        }
        if is_file {
            id += 1;
        }
        is_file = !is_file;
    }
    let mut q = filesystem.len() - 1;
    'outer: while q > 0 {
        if let Block::Empty = filesystem[q] {
            q -= 1;
            continue;
        }
        let mut q_end = q;
        while q_end > 0 && filesystem[q_end - 1] == filesystem[q] {
            q_end -= 1;
        }
        if q_end == 0 {
            break;
        }
        let file_size = q + 1 - q_end;
        let mut p = 0;
        while p < q_end {
            if filesystem[p] != Block::Empty {
                p += 1;
                continue;
            }
            let mut p_end = p;
            while p_end < filesystem.len() - 1 && filesystem[p_end + 1] == filesystem[p] {
                p_end += 1;
            }
            let empty_size = p_end + 1 - p;
            if empty_size >= file_size {
                for i in 0..file_size {
                    filesystem[p + i] = filesystem[q_end + i].clone();
                    filesystem[q_end + i] = Block::Empty
                }
                q = if q_end == 0 { 0 } else { q_end - 1 };
                continue 'outer;
            } else {
                p = p_end + 1;
                continue;
            }
        }
        q = if q_end == 0 { 0 } else { q_end - 1 };
        
    }
    for i in 0..filesystem.len() {
        match filesystem[i] {
            Block::Empty => continue,
            Block::File(id) => sum += i as u64 * id,
        }
    }
    println!("{:?}", filesystem);
    println!("{:?}", sum);

}

#[derive(Debug, Clone, PartialEq, Hash)]
enum Block {
    Empty,
    File(u64),
}
