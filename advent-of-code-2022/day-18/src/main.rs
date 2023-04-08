use std::{collections::{HashSet, VecDeque}, fs, time::Instant};

fn main() {
    first_problem();
    second_problem();
}

fn second_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut cubes: Vec<(i8, i8, i8)> = Vec::new();
    let mut cube_set: HashSet<(i8, i8, i8)> = HashSet::new();

    let mut min_x: i8 = 127;
    let mut min_y: i8 = 127;
    let mut min_z: i8 = 127;

    let mut max_x: i8 = -128;
    let mut max_y: i8 = -128;
    let mut max_z: i8 = -128;

    for line in data.split("\n") {
        let numbers: Vec<&str> = line.split(",").collect();
        let (x, y, z) = (
            numbers[0].parse::<i8>().unwrap(),
            numbers[1].parse::<i8>().unwrap(),
            numbers[2].parse::<i8>().unwrap(),
        );
        if min_x > x {
            min_x = x;
        }
        if min_y > y {
            min_y = y;
        }
        if min_z > z {
            min_z = z;
        }
        if max_x < x {
            max_x = x;
        }
        if max_y < y {
            max_y = y;
        }
        if max_z < z {
            max_z = z;
        }
        cubes.push((x, y, z));
        cube_set.insert((x, y, z));
    }
    println!("min_x: {}, max_x: {}", min_x, max_x);
    println!("min_y: {}, max_y: {}", min_y, max_y);
    println!("min_z: {}, max_z: {}", min_z, max_z);

    min_x -= 1;
    min_y -= 1;
    min_z -= 1;

    max_x += 1;
    max_y += 1;
    max_z += 1;

    let mut air_set: HashSet<(i8, i8, i8)> = HashSet::new();
    let mut air_deque: VecDeque<(i8,i8,i8)> = VecDeque::new();

    air_set.insert((min_x, min_y, min_z));
    air_deque.push_back((min_x, min_y, min_z));
    while !air_deque.is_empty() {
        let air = air_deque.pop_front().unwrap();
        let neighbours = get_neighbours_with_boundaries(air.0, air.1, air.2, min_x, max_x, min_y, max_y, min_z, max_z);
        for neighbour in neighbours {
            if !air_set.contains(&neighbour) && !cube_set.contains(&neighbour) {
                air_set.insert(neighbour);
                air_deque.push_back(neighbour);
            }
        }
    }

    let mut total_sides: u32 = 0;
    for cube in &cubes {
        for neighbour in get_neighbours(cube.0, cube.1, cube.2) {
            if !cube_set.contains(&neighbour) && air_set.contains(&neighbour) {
                total_sides += 1;
            }
        }
    }
    println!("{}", total_sides);
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn first_problem() {
    let now = Instant::now();
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();
    let mut cubes: Vec<(i8, i8, i8)> = Vec::new();
    let mut cube_set: HashSet<(i8, i8, i8)> = HashSet::new();

    for line in data.split("\n") {
        let numbers: Vec<&str> = line.split(",").collect();
        let (x, y, z) = (
            numbers[0].parse::<i8>().unwrap(),
            numbers[1].parse::<i8>().unwrap(),
            numbers[2].parse::<i8>().unwrap(),
        );
        cubes.push((x, y, z));
        cube_set.insert((x, y, z));
    }
    let mut total_sides: u32 = 0;
    for cube in &cubes {
        for neighbour in get_neighbours(cube.0, cube.1, cube.2) {
            if !cube_set.contains(&neighbour) {
                total_sides += 1;
            }
        }
    }
    println!("{}", total_sides);
    let elapsed = now.elapsed();
    println!("Elapsed part 1: {:.2?}", elapsed);
}

fn get_neighbours(x: i8, y: i8, z: i8) -> Vec<(i8, i8, i8)> {
    return vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
}

fn get_neighbours_with_boundaries(
    x: i8,
    y: i8,
    z: i8,
    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
    min_z: i8,
    max_z: i8,
) -> Vec<(i8, i8, i8)> {
    let vec = vec![
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ];
    return vec
        .into_iter()
        .filter(|point| {
            point.0 >= min_x
                && point.0 <= max_x
                && point.1 >= min_y
                && point.1 <= max_y
                && point.2 >= min_z
                && point.2 <= max_z
        })
        .collect();
}
