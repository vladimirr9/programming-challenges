use advent_of_code_2024::utils::parsing_utils;
use regex::Regex;

fn main() {
    let content = parsing_utils::get_contents(13, true, 1).trim().to_owned();
    let machines: Vec<String> = content
        .split("\n\n")
        .map(|machine| machine.trim().to_owned())
        .collect();

    // for machine in machines.iter() {
    //     println!("{}", machine)
    // }

    // let regex = Regex::new(
    //     r"Button A: X\+(\d+), Y\+(\d+)\s*
    //       Button B: X\+(\d+), Y\+(\d+)\s*
    //       Prize: X=(\d+), Y=(\d+)"
    // ).unwrap();

    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\n\s*Button B: X\+(\d+), Y\+(\d+)\n\s*Prize: X=(\d+), Y=(\d+)"
    ).unwrap();

    for machine in machines {
       let  res = regex.captures(machine.as_ref()).unwrap();
        let (A_x, A_y, B_x, B_y, target_x, target_y) = (
            res.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(5).unwrap().as_str().parse::<u32>().unwrap(),
            res.get(6).unwrap().as_str().parse::<u32>().unwrap(),
        );


    }



}


struct State {
    x: u32,
    y: u32,
    x_pressed: u32,
    y_pressed: u32
}