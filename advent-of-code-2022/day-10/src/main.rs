use std::fs;

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    first_problem(data);

    let mut register = 1;
    let mut counter = 0;
    for line in data.split("\n") {

        if line.starts_with("noop") {
            print_pixel(counter, register);
            counter += 1;
        } else {
            let (_, instruction_value) = line.split_once(" ").unwrap();
            let instruction_value: i32 = instruction_value.trim().parse().unwrap();
            print_pixel(counter, register);
            counter += 1;
            print_pixel(counter, register);
            counter += 1;

            register += instruction_value;
        }
    }

}


fn print_pixel(counter: i32, register: i32) {
    let counter_val_adjusted = counter % 40;
    if counter != 0 && counter_val_adjusted == 0 {
        println!();
    }
    if register - 1 == counter_val_adjusted || register == counter_val_adjusted || register + 1 == counter_val_adjusted {
        print!("#");
    } else {
        print!(".");
    }

}

fn first_problem(data: &str) {
    let counter_points_of_interest: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut total_sum = 0;
    let mut register = 1;
    let mut counter = 0;
    for line in data.split("\n") {

        if line.starts_with("noop") {
            counter += 1;
            check_counter_value(counter_points_of_interest, counter, register, &mut total_sum);
        } else {
            let (_, instruction_value) = line.split_once(" ").unwrap();
            let instruction_value: i32 = instruction_value.trim().parse().unwrap();
            counter += 1;
            check_counter_value(counter_points_of_interest, counter, register, &mut total_sum);
            counter += 1;
            check_counter_value(counter_points_of_interest, counter, register, &mut total_sum);
            register += instruction_value;
        }
    }
    println!("{total_sum}");
}

fn check_counter_value(counter_points_of_interest: [i32; 6], counter: i32, register: i32, total_sum: &mut i32) {
    if counter_points_of_interest.contains(&counter) {
        let strength = counter * register;
        *total_sum += strength;
        println!("Counter: {}, value: {}", counter, strength)
    }
}


