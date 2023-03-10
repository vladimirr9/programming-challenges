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

    first_problem(data);
    second_problem(data);
}


fn second_problem(data: &str) {
    let (terrain, _start, end) = assemble_terrain(data);
    let width = terrain[0].len();
    let height = terrain.len();
    let terrain = Terrain {
        values: terrain,
        width: width,
        height: height,
    };

    let mut a_nodes: Vec<(usize, usize)> = Vec::new();
    let mut a_node_distances: Vec<u32> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            if terrain.values[i][j] == 0 {
                a_nodes.push((i,j));
            }
        }
    }


    while !a_nodes.is_empty() {
        let start = a_nodes.pop().unwrap();
        let mut unvisited_nodes: HashSet<(usize, usize)> = HashSet::new();
        let mut tentative_distances: Vec<Vec<u32>> = Vec::new();
        for i in 0..height {
            tentative_distances.push(Vec::new());
            for j in 0..width {
                tentative_distances[i].push(u32::MAX - 1);
                unvisited_nodes.insert((i, j));
            }
        }
        
        tentative_distances[start.0][start.1] = 0;
        let mut current_node = start;
        while unvisited_nodes.contains(&end) {
            for node in get_unvisited_walkable_neighbors(
                &terrain,
                &unvisited_nodes,
                current_node.0,
                current_node.1,
            ) {
                let potential_tentative_difference =
                    tentative_distances[current_node.0][current_node.1] + 1;
                if potential_tentative_difference < tentative_distances[node.0][node.1] {
                    tentative_distances[node.0][node.1] = potential_tentative_difference;
                }
            }
            unvisited_nodes.remove(&current_node);
    
            if unvisited_nodes.len() == 0 {
                break;
            }
            current_node = get_unvisited_node_with_lowest_tentative_distance(
                &tentative_distances,
                &unvisited_nodes,
            )
            .expect("There must be at least one node that's unvisited");
        }
    
        // println!("{:?}", tentative_distances);
        // println!("{}", tentative_distances[end.0][end.1]);
        a_node_distances.push(tentative_distances[end.0][end.1])
    }
    println!("{}", a_node_distances.iter().min().unwrap());

    
}


fn first_problem(data: &str) {
    let (terrain, start, end) = assemble_terrain(data);
    let width = terrain[0].len();
    let height = terrain.len();
    let terrain = Terrain {
        values: terrain,
        width: width,
        height: height,
    };

    let mut unvisited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut tentative_distances: Vec<Vec<u32>> = Vec::new();
    for i in 0..height {
        tentative_distances.push(Vec::new());
        for j in 0..width {
            tentative_distances[i].push(u32::MAX - 1);
            unvisited_nodes.insert((i, j));
        }
    }
    tentative_distances[start.0][start.1] = 0;
    let mut current_node = start;
    while unvisited_nodes.contains(&end) {
        for node in get_unvisited_walkable_neighbors(
            &terrain,
            &unvisited_nodes,
            current_node.0,
            current_node.1,
        ) {
            let potential_tentative_difference =
                tentative_distances[current_node.0][current_node.1] + 1;
            if potential_tentative_difference < tentative_distances[node.0][node.1] {
                tentative_distances[node.0][node.1] = potential_tentative_difference;
            }
        }
        unvisited_nodes.remove(&current_node);

        if unvisited_nodes.len() == 0 {
            break;
        }
        current_node = get_unvisited_node_with_lowest_tentative_distance(
            &tentative_distances,
            &unvisited_nodes,
        )
        .expect("There must be at least one node that's unvisited");
    }

    // println!("{:?}", tentative_distances);
    println!("{}", tentative_distances[end.0][end.1]);
}

fn get_unvisited_walkable_neighbors(
    terrain: &Terrain,
    unvisited_nodes: &HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> HashSet<(usize, usize)> {
    let adjacent: Vec<(usize, usize)> = vec![
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
    .filter(|val| unvisited_nodes.contains(val))
    .filter(|val| terrain.values[val.0][val.1] <= terrain.values[i][j] + 1)
    .collect();
    return HashSet::from_iter(adjacent);
}

fn get_unvisited_node_with_lowest_tentative_distance(
    tentative_distances: &Vec<Vec<u32>>,
    unvisited_nodes: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let mut node: Option<(usize, usize)> = None;
    let mut node_tentative_distance = u32::MAX;
    for (i, row) in tentative_distances.iter().enumerate() {
        for (j, _val) in row.iter().enumerate() {
            if !unvisited_nodes.contains(&(i, j)) {
                continue;
            }
            if tentative_distances[i][j] <= node_tentative_distance {
                node_tentative_distance = tentative_distances[i][j];
                node = Some((i, j));
            }
        }
    }
    return node;
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
            row.push(value);
        }
    }
    return (terrain, start, end);
}
