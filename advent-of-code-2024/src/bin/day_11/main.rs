use advent_of_code_2024::utils::parsing_utils;

fn main() {
    let contents = parsing_utils::get_contents(11, false, 1).trim().to_owned();
    let mut nums: Vec<u64> = contents.split_ascii_whitespace().map(|val| val.parse().unwrap()).collect();
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



fn apply_blink_rule(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }    
    let num_string = num.to_string();
    if num_string.len() % 2 == 0 {
        let (left, right) = num_string.split_at(num_string.len() / 2);
        return [left, right].iter().map(|val| val.parse::<u64>().unwrap()).collect()
    }
    return vec![num * 2024];
}