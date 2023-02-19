use petgraph::{dot::Dot, Graph};
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashSet<(u32, String)>,
    size: i32,
}

fn main() {
    let filepath = "input.txt";
    let binding = fs::read_to_string(filepath).expect("Should be able to read file");
    let data = binding.trim();
    let mut file_system: Graph<Directory, u8> = Graph::<Directory, u8>::new();
    let base_directory = Directory {
        name: String::from("/"),
        files: HashSet::new(),
        size: -1,
    };
    let mut current_index = file_system.add_node(base_directory);

    for line in data.split("\n") {
        println!("{line}");
        if is_command(line) {
            println!("{:?}", Dot::new(&file_system));
            let (_, statement) = line.split_once(" ").unwrap();
            let statement = statement.trim();
            if statement.starts_with("cd") {
                let (_, destination) = statement.split_once(" ").unwrap();
                match destination {
                    "/" => {
                        current_index = file_system
                            .node_indices()
                            .find(|node_index| file_system[*node_index].name == "/")
                            .unwrap();
                        continue;
                    }
                    ".." => {
                        current_index = file_system
                            .neighbors_undirected(current_index)
                            .find(|node| file_system.contains_edge(*node, current_index))
                            .unwrap();
                        continue;
                    }
                    _ => {
                        current_index = file_system
                            .neighbors(current_index)
                            .find(|node| file_system[*node].name == destination)
                            .unwrap();
                        continue;
                    }
                }
            } else if statement.starts_with("ls") {
                continue;
            }
        } else if line.starts_with("dir") {
            let (_, directory_name) = line.split_once(" ").unwrap();
            let new_dir = Directory {
                name: String::from(directory_name),
                files: HashSet::new(),
                size: -1,
            };
            let dir_ind = file_system.add_node(new_dir);
            file_system.add_edge(current_index, dir_ind, 1);
        } else {
            let (size, file_name) = line.split_once(" ").unwrap();
            let size: u32 = size.parse().unwrap();
            let files = &mut file_system[current_index].files;
            files.insert((size, String::from(file_name)));
        }
    }
    // bad way to iterate and calculate the size of every directory, but it works.
    while file_system
        .node_indices()
        .any(|node_index| file_system[node_index].size == -1)
    {
        for node_index in file_system.node_indices() {
            if file_system
                .neighbors(node_index)
                .any(|node_index| file_system[node_index].size == -1)
            {
                continue;
            }
            let mut neighbour_size = 0;
            for neighbour_node_index in file_system.neighbors(node_index) {
                neighbour_size += &file_system[neighbour_node_index].size;
            }
            let node = &mut file_system[node_index];
            let size = &mut node.size;
            *size = node
                .files
                .iter()
                .map(|file| file.0)
                .reduce(|acc, e| acc + e)
                .unwrap_or(0) as i32
                + neighbour_size;
        }
    }
    let mut total_sum = 0;
    for node_index in file_system.node_indices() {
        let node = &file_system[node_index];
        if node.size < 100_000 {
            total_sum += node.size;
        }
    }
    println!("{:?}", Dot::new(&file_system));
    println!("Total sum {}", total_sum);

    // problem 2
    let total_capacity = 70_000_000;
    let needed_capacity = 30_000_000;
    let root = file_system.node_indices().find(|node_index| file_system[*node_index].name == "/").unwrap();
    let used_space = file_system[root].size;
    let minimum_space_to_remove = needed_capacity - (total_capacity - used_space);
    let mut minimum = &file_system[root];
    for node_index in file_system.node_indices() {
        let node = &file_system[node_index];
        if node.size < minimum_space_to_remove {
            continue;
        }
        if node.size < minimum.size {
            minimum = node;
        }
    }
    println!("Should remove directory {} with size {}", minimum.name, minimum.size);
}

fn is_command(line: &str) -> bool {
    line.chars().next().unwrap() == '$'
}
