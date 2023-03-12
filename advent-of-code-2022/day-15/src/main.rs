use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance_to_beacon: u64,
}

struct Beacon {
    x: i64,
    y: i64,
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Beacon> = Vec::new();
    println!("{}", data);
    for line in data.split("\n") {
        let numbers = str_strip_numbers(line);
        let beacon = Beacon {
            x: numbers[2],
            y: numbers[3],
        };
        let sensor = Sensor {
            x: numbers[0],
            y: numbers[1],
            distance_to_beacon: manhattan_distance(numbers[0], numbers[1], beacon.x, beacon.y),
        };
        println!("{:?}", sensor);
        beacons.push(beacon);
        sensors.push(sensor);
    }
    println!("{}", count_positions(2000000, &sensors, &beacons));
}

fn count_positions(row: i64, sensors: &Vec<Sensor>, beacons: &Vec<Beacon>) -> u64 {
    let mut positions: u64 = 0;
    'outer: for i in -20_000_000_i64..20_000_000 {
        if beacons.iter().any(|beacon| beacon.x == i && beacon.y == row) {
            continue;
        }
        for sensor in sensors {
            if manhattan_distance(i, row, sensor.x, sensor.y) <= sensor.distance_to_beacon {
                positions += 1;
                continue 'outer;
            }
        }
    }
    return positions;
}

fn str_strip_numbers(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }
    return RE
        .find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();
}

fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> u64 {
    return ((x2 - x1).abs() + (y2 - y1).abs()) as u64;
}
