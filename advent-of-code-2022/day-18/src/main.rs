use std::{
    collections::{HashSet},
    fs, time::Instant,
};

fn main() {
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
    println!("Elapsed: {:.2?}", elapsed);
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
