use advent_of_code_2024::utils::parsing_utils;
use regex::Regex;

fn main() {
    // first_problem();
    second_problem();
}

fn first_problem() {
    let mut sum = 0;
    let contents = parsing_utils::get_contents(3, false, 1).trim().to_owned();
    let pattern = r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    for capture in re.captures_iter(&contents) {
        let first = capture["first"].parse::<u32>().unwrap();
        let second = capture["second"].parse::<u32>().unwrap();
        sum += first * second;
    }

    println!("{}", sum);
}


fn second_problem() {
    let mut sum = 0;
    let contents = parsing_utils::get_contents(3, false, 2).trim().to_owned();
    let pattern = r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    let mut parts = contents.split("don't()");
    for capture in re.captures_iter(&parts.next().unwrap()) {
        let first = capture["first"].parse::<u32>().unwrap();
        let second = capture["second"].parse::<u32>().unwrap();
        sum += first * second;
    }
    for part in parts {
        let valid_part = &part[part.find("do()").unwrap_or(part.len())..];
        for capture in re.captures_iter(valid_part) {
            let first = capture["first"].parse::<u32>().unwrap();
            let second = capture["second"].parse::<u32>().unwrap();
            sum += first * second;
        }
    }

    println!("{}", sum);
}
