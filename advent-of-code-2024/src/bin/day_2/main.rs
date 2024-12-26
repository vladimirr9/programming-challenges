use advent_of_code_2024::utils::parsing_utils::get_contents;

fn main() {
    let contents = get_contents(2, false).trim().to_owned();
    let allowed_distances: [u32; 3] = [1, 2, 3];
    print!("{}", &contents);
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut num_are_safe = 0;
    for line in lines {
        let line: Vec<u32> = line
            .split_ascii_whitespace()
            .map(|val| val.parse::<u32>().unwrap())
            .collect();
        if is_asc_or_desc(&line) && is_correct_distance(&line, &allowed_distances) {
            num_are_safe += 1;
        }
    }
    print!("{num_are_safe}")
}

fn is_line_safe(line: &Vec<u32>, allowed_distances: &[u32]) -> bool {
    is_asc_or_desc(line)
}

fn is_asc_or_desc(line: &Vec<u32>) -> bool {
    line.windows(2).all(|vals| vals[0] > vals[1]) || line.windows(2).all(|vals| vals[0] < vals[1])
}

fn is_correct_distance(line: &Vec<u32>, allowed_distances: &[u32]) -> bool {
    line.windows(2)
        .all(|vals| allowed_distances.contains(&vals[0].abs_diff(vals[1])))
}
