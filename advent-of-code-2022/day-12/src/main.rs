use std::{collections::HashSet, fs, ops::IndexMut};

struct Terrain {
    values: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

fn main() {
    let filepath = "input.txt";
    let data = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = data.trim();

    let (terrain, start, end) = assemble_terrain(data);
    let width = terrain[0].len();
    let height = terrain.len();
    let terrain = Terrain {
        values: terrain,
        width: width,
        height: height,
    };
    // for ele in get_available_moves(&terrain, 0, 0) {
    //     println!("{}, {}", ele.0, ele.1);
    // }

    let mut trodden_paths: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut start_visited_locations: Vec<(usize, usize)> = Vec::new();
    start_visited_locations.push(start);

    let mut initial_path: Vec<(usize, usize)> = Vec::new();
    initial_path.push(start);

    while get_available_untrodden_moves(
        &start_visited_locations,
        &trodden_paths,
        &get_available_moves(&terrain, start.0, start.1),
    )
    .len()
        > 0
    {
        let mut visited_locations: Vec<(usize, usize)> = Vec::new();
        visited_locations.push(start);
        let mut position = start;
        let mut new_path: Vec<(usize, usize)> = Vec::new();
        new_path.push(position);
        loop {
            let mut potential_new_paths = get_available_untrodden_moves(
                &visited_locations,
                &trodden_paths,
                &get_available_moves(&terrain, position.0, position.1),
            );
            if potential_new_paths.is_empty() {
                for ele in &new_path {
                    // print!("({}, {}), ", ele.0, ele.1);
                }
                // println!();
                trodden_paths.push(new_path);
                break;
            }
            new_path = potential_new_paths.pop().unwrap();
            position = *new_path.last().unwrap();
            visited_locations.push(position);
            // print!("({}, {}), ", position.0, position.1);
        }
    }
    let mut paths_containing_end : Vec<&Vec<(usize, usize)>> = trodden_paths.iter().filter(|path| path.contains(&end)).collect();
    paths_containing_end.sort_by(|path_1, path_2| path_1.len().cmp(&path_2.len()));
    let solution = paths_containing_end.first().unwrap();
    println!("Solution steps: {}", solution.len() - 1);
    for position in *solution {
        print!("({}, {}), ", position.0, position.1);
    }
    println!();
    println!("Dimensions: {}, {}", width, height);
}

fn assemble_terrain(data: &str) -> (Vec<Vec<u32>>, (usize, usize), (usize, usize)) {
    let mut terrain: Vec<Vec<u32>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (i, line) in data.split("\n").enumerate() {
        terrain.push(Vec::new());
        let row = terrain.index_mut(i);
        for (j, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (i.try_into().unwrap(), j.try_into().unwrap());
                row.push(0);
                continue;
            }
            if char == 'E' {
                end = (i.try_into().unwrap(), j.try_into().unwrap());
                row.push(25);
                continue;
            }
            let value = char.to_ascii_lowercase() as u32 - 97;
            println!("{} - coords : {}, {}", value, i, j);
            row.push(value);
        }
    }
    return (terrain, start, end);
}

fn get_available_moves(terrain: &Terrain, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut adjacent: Vec<(usize, usize)> = vec![
        (i as i32 + 1, j as i32),
        (i as i32 - 1, j as i32),
        (i as i32, j as i32 + 1),
        (i as i32, j as i32 - 1),
    ]
    .into_iter()
    .filter(|val| {
        val.0 >= 0
            && val.0 < terrain.height.try_into().unwrap()
            && val.1 >= 0
            && val.1 < terrain.width.try_into().unwrap()
    })
    .map(|val| {
        (
            usize::try_from(val.0).unwrap(),
            usize::try_from(val.1).unwrap(),
        )
    })
    .filter(|val| terrain.values[val.0][val.1] <= terrain.values[i][j] + 1)
    .collect();
    return adjacent;
}

fn get_available_untrodden_moves(
    visited_locations: &Vec<(usize, usize)>,
    trodden_paths: &Vec<Vec<(usize, usize)>>,
    available_moves: &Vec<(usize, usize)>,
) -> Vec<Vec<(usize, usize)>> {
    let mut available_untrodden_moves: Vec<Vec<(usize, usize)>> = Vec::new();
    for available_move in available_moves {
        let mut new_path = visited_locations.clone();
        if visited_locations.contains(available_move) {
            continue;
        }
        new_path.push(*available_move);
        // if !trodden_paths.iter().any(|trodden_path| *trodden_path == new_path) {
        if !trodden_paths.contains(&new_path) {
            available_untrodden_moves.push(new_path);
        }
    }
    return available_untrodden_moves;
}
