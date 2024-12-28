use std::collections::{HashMap, HashSet};

use advent_of_code_2024::utils::parsing_utils;

fn main() {
    // first_problem();
    second_problem();
    }
    
    fn first_problem() {
    use std::time::Instant;
    let now = Instant::now();
    let content = parsing_utils::get_contents(5, false, 1);
    let (rules, pages) = content.split_once("\n\n").unwrap();
    let rules = rules.trim();
    let pages = pages.trim();
    // key is a number and value is a set of all numbers that MUST go BEFORE it
    let mut rule_map : HashMap<String, HashSet<String>> = HashMap::new();
    for entry in rules.lines() {
        let (key,value) = entry.split_once("|").unwrap();
        if rule_map.get(value).is_none() {
            rule_map.insert(value.to_string(), HashSet::new());
        }
        let set = rule_map.get_mut(value).unwrap();
        set.insert(key.to_string());
    }
    let mut sum = 0;
    
    for page in pages.lines() {
        let page_numbers = page.split(",").map(|val| val.to_string()).collect::<Vec<String>>();
        if is_order_correct(&page_numbers, &rule_map) {
            sum += page_numbers[page_numbers.len()/2].parse::<u32>().unwrap()
        }
    }
    println!("{sum}");
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    }


    fn second_problem() {
        use std::time::Instant;
        let now = Instant::now();
        let content = parsing_utils::get_contents(5, false, 1);
        let (rules, pages) = content.split_once("\n\n").unwrap();
        let rules = rules.trim();
        let pages = pages.trim();
        // key is a number and value is a set of all numbers that MUST go BEFORE it
        let mut rule_map: HashMap<String, HashSet<String>> = HashMap::new();
        for entry in rules.lines() {
            let (key, value) = entry.split_once("|").unwrap();
            if rule_map.get(value).is_none() {
                rule_map.insert(value.to_string(), HashSet::new());
            }
            let set = rule_map.get_mut(value).unwrap();
            set.insert(key.to_string());
        }
        let mut sum = 0;
    
        for page in pages.lines() {
            let page_numbers = page
                .split(",")
                .map(|val| val.to_string())
                .collect::<Vec<String>>();
            if is_order_correct(&page_numbers, &rule_map) {
                continue;
            }
            let correct_order = correct_order(&page_numbers, &rule_map);
            sum += correct_order[correct_order.len()/2].parse::<u32>().unwrap();
        }
        println!("{sum}");
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    
    fn correct_order(page_numbers: &Vec<String>, rule_map: &HashMap<String, HashSet<String>>) -> Vec<String> {
        let mut correct_order: Vec<String> = Vec::new();
        let mut page_numbers_vec: Vec<String> = page_numbers.iter().map(|str| str.to_owned()).collect();
        'outer: while !page_numbers_vec.is_empty() {
            'inner: for i in 0..page_numbers_vec.len() {
                let number = &page_numbers_vec[i];
                for remaining_number in page_numbers_vec.iter() {
                    if remaining_number == number {
                        continue;
                    }
                    let before_required_numbers = rule_map.get(remaining_number);
                    if before_required_numbers.is_some() && before_required_numbers.unwrap().contains(number) {
                        continue 'inner;
                    }
                }
                correct_order.insert(0, number.to_owned());
                page_numbers_vec.swap_remove(i);
                continue 'outer;
            }
        }
        return correct_order;
    }
    

fn is_order_correct(page_numbers: &Vec<String>, rule_map: &HashMap<String, HashSet<String>>) -> bool {
    if page_numbers.len() <= 1 {
        return true;
    }
    let mut latter_numbers: HashSet<String> = HashSet::new();
    for number in page_numbers.iter().rev() {
        latter_numbers.insert(number.to_string());
        let Some(rule_map_entry) = rule_map.get(number) else {
            continue;
        };
        if latter_numbers.intersection(rule_map_entry).next().is_some() {
            return false
        }
    }
    true
}