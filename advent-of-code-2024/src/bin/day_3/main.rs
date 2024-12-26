use advent_of_code_2024::utils::parsing_utils;
use regex::Regex;

fn main() {
    let mut sum = 0;
    let contents = parsing_utils::get_contents(3, false).trim().to_owned();
    let pattern = r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();

    for capture in re.captures_iter(&contents) {
        let first = capture["first"].parse::<u32>().unwrap();
        let second = capture["second"].parse::<u32>().unwrap();
        sum += first * second;
    }

    println!("{}", sum);
}