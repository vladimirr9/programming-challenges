use advent_of_code_2024::utils::parsing_utils;
use ahash::AHashSet;

fn main() {
    let content = parsing_utils::get_contents(12, false, 1);
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let height = matrix.len();
    let width = matrix[0].len();
    let mut regions: Vec<Region> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            if regions.iter().any(|region| region.contains_plot(i, j)) {
                continue;
            } else {
                let region = get_region(&matrix, i, j, AHashSet::new());
                regions.push(region);
            }
        }
    }
    let mut sum = 0;
    for region in regions.iter() {
        sum += region.get_price();
        println!("{:?}", region)
    }
    println!("{}", sum);
}

fn get_region(matrix: &Vec<Vec<char>>, x: usize, y: usize, existing_region_plots: AHashSet<(usize, usize)>) -> Region {
    let char = matrix[x][y];
    let mut plots: AHashSet<(usize, usize)> = existing_region_plots.clone();
    plots.insert((x,y));
    let mut perimeter = 4;
    let neighbours = get_valid_neighbours(matrix, x, y);
    for neighbour in neighbours {
        if matrix[neighbour.0][neighbour.1] == char {
            perimeter -= 1;
        }
    }
    let mut region = Region{
        plots: plots.clone(),
        char,
        perimeter: perimeter,
    };
    for neighbour in get_valid_neighbours(matrix, x, y) {
        if matrix[neighbour.0][neighbour.1] == char && !region.plots.contains(&neighbour) {
            region = region.clone().merge(get_region(matrix, neighbour.0, neighbour.1, region.clone().plots));
        }
    }
    return region;
}

fn get_valid_neighbours(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for (x_change, y_change) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let potential_x = i as i32 + x_change;
        let potential_y = j as i32 + y_change;
        if potential_x >= 0
            && potential_x < height as i32
            && potential_y >= 0
            && potential_y < width as i32
        {
            neighbours.push((potential_x as usize, potential_y as usize));
        }
    }
    return neighbours;
}

#[derive(Debug, Clone)]
struct Region {
    plots: AHashSet<(usize, usize)>,
    char: char,
    perimeter: u32,
}

impl Region {
    fn merge(self, region: Region) -> Region {
        return Region{
            plots: self.plots.union(&region.plots).map(|val| val.to_owned()).collect(),
            char: self.char,
            perimeter: self.perimeter + region.perimeter,
        };
    }
    fn contains_plot(&self, x: usize, y: usize) -> bool {
        return self.plots.contains(&(x, y));
    }
    fn get_area(&self) -> usize {
        return self.plots.len();
    }
    fn get_price(&self) -> u32 {
        return self.get_area() as u32 * self.perimeter
    }
}
