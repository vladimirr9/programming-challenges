use advent_of_code_2024::utils::parsing_utils;
use ahash::AHashMap;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    // first_problem();
    second_problem();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn first_problem() {
    let contents = parsing_utils::get_contents(11, false, 1).trim().to_owned();
    let mut nums: Vec<u64> = contents
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
    for i in 0..25 {
        let mut tmp: Vec<u64> = Vec::new();
        for num in nums.iter() {
            let mut new_nums = apply_blink_rule(*num);
            tmp.append(&mut new_nums);
        }
        nums = tmp;
    }
    println!("{:?}", nums);
    println!("{:?}", nums.len());
}

fn second_problem() {
    let contents = parsing_utils::get_contents(11, false, 1).trim().to_owned();
    let mut cache: AHashMap<u64, Vec<u64>> = AHashMap::new();
    let nums: Vec<u64> = contents
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();

    let mut tmp: AHashMap<u64, u64> = AHashMap::new();
    for num in nums {
        tmp.entry(num).and_modify(|val| *val += 1).or_insert(1);
    }
    let mut nums: AHashMap<u64, u64> = tmp;
    for _i in 0..75 {
        // println!("{i}");
        let mut tmp: AHashMap<u64, u64> = AHashMap::new();
        for (key, value) in nums.iter() {
            let new_nums: &Vec<u64> = cache.entry(*key).or_insert_with(|| apply_blink_rule(*key));
            for new_num in new_nums {
                tmp.entry(*new_num)
                    .and_modify(|val| *val += value)
                    .or_insert(*value);
            }
        }
        nums = tmp;
    }
    let mut sum = 0;
    for value in nums.values() {
        sum += value
    }
    println!("{sum}")
}

fn apply_blink_rule(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }
    let num_string = num.to_string();
    if num_string.len() % 2 == 0 {
        let (left, right) = num_string.split_at(num_string.len() / 2);
        return [left, right]
            .iter()
            .map(|val| val.parse::<u64>().unwrap())
            .collect();
    }
    return vec![num * 2024];
}
