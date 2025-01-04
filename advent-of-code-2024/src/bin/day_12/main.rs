
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
    let mut sum = 0;
    for region in regions.iter() {
        sum += region.get_bulk_discount_price(&matrix);
        println!("{:?}", region);
        println!("{}", region.get_bulk_discount_price(&matrix));
    }
    println!("{}", sum);
}

fn get_region(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    existing_region_plots: AHashSet<(usize, usize)>,
) -> Region {
    let char = matrix[x][y];
    let mut plots: AHashSet<(usize, usize)> = existing_region_plots.clone();
    plots.insert((x, y));
    let mut perimeter = 4;
    let neighbours = get_valid_neighbours(matrix, x, y);
    for neighbour in neighbours {
        if matrix[neighbour.0][neighbour.1] == char {
            perimeter -= 1;
        }
    }
    let mut region = Region {
        plots: plots.clone(),
        char,
        perimeter: perimeter,
    };
    for neighbour in get_valid_neighbours(matrix, x, y) {
        if matrix[neighbour.0][neighbour.1] == char && !region.plots.contains(&neighbour) {
            region = region.clone().merge(get_region(
                matrix,
                neighbour.0,
                neighbour.1,
                region.clone().plots,
            ));
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
        return Region {
            plots: self
                .plots
                .union(&region.plots)
                .map(|val| val.to_owned())
                .collect(),
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
        return self.get_area() as u32 * self.perimeter;
    }

    fn get_bulk_discount_price(&self, matrix: &Vec<Vec<char>>) -> u32 {
        let mut perimeters: AHashSet<Perimeter> = AHashSet::new();
        let height = matrix.len();
        let width = matrix[0].len();
        for plot in self.plots.clone() {
            let (x, y) = plot;
            let right_char = get_char(matrix, x as i32, y as i32 + 1);
            let bottom_char = get_char(matrix, x as i32 + 1, y as i32);
            let left_char = get_char(matrix, x as i32, y as i32 - 1);
            let top_char = get_char(matrix, x as i32 - 1, y as i32);
            if right_char.is_none() || right_char.unwrap() != self.char {
                perimeters.insert(Perimeter {
                    x,
                    y,
                    direction: Direction::RIGHT,
                });
            }
            if left_char.is_none() || left_char.unwrap() != self.char {
                perimeters.insert(Perimeter {
                    x,
                    y,
                    direction: Direction::LEFT,
                });
            }
            if top_char.is_none() || top_char.unwrap() != self.char {
                perimeters.insert(Perimeter {
                    x,
                    y,
                    direction: Direction::UP,
                });
            }
            if bottom_char.is_none() || bottom_char.unwrap() != self.char {
                perimeters.insert(Perimeter {
                    x,
                    y,
                    direction: Direction::DOWN,
                });
            }
        }
        let mut sides = 0;
        while !perimeters.is_empty() {
            let val = perimeters.iter().next().unwrap().clone();
            let perimeter = perimeters.take(&val).unwrap();
            sides += 1;
            let (x, y) = (perimeter.x, perimeter.y);
            let direction = &perimeter.direction.clone();
            let (i, j) = match direction {
                Direction::UP | Direction::DOWN => (0, 1),
                Direction::RIGHT | Direction::LEFT => (1, 0),
            };
            for iter in 1.. {
                let next_x = x as i32 + iter * i;
                if next_x < 0 || next_x >= height as i32 {
                    break;
                }
                let next_y = y as i32 + iter * j;
                if next_y < 0 || next_y >= width as i32 {
                    break;
                }

                let expected_perimeter = Perimeter {
                    x: next_x as usize,
                    y: next_y as usize,
                    direction: direction.clone(),
                };
                let perimeter = perimeters.take(&expected_perimeter);
                match perimeter {
                    Some(_) => {
                    }
                    None => {
                        break;
                    }
                }
            }
            for iter in 1.. {
                let next_x = x as i32 - iter * i;
                if next_x < 0 || next_x >= height as i32 {
                    break;
                }
                let next_y = y as i32 - iter * j;
                if next_y < 0 || next_y >= width as i32 {
                    break;
                }

                match perimeters.take(&Perimeter {
                    x: next_x as usize,
                    y: next_y as usize,
                    direction: direction.clone(),
                }) {
                    Some(_) => {}
                    None => break,
                }
            }
        }

        return sides as u32 * self.get_area() as u32;
    }
    //     let mut visited_plots: AHashSet<(usize, usize)> = AHashSet::new();
    //     let mut sides = 0;
    //     for plot in self.plots.clone() {
    //         let (x, y) = plot;
    //         visited_plots.insert((x, y));
    //         let right_char = get_char(matrix, x as i32, y as i32 + 1);
    //         let bottom_char = get_char(matrix, x as i32 + 1, y as i32);
    //         let left_char = get_char(matrix, x as i32, y as i32 - 1);
    //         let top_char = get_char(matrix, x as i32 - 1, y as i32);

    //         let bottom_left_char = get_char(matrix, x as i32 + 1, y as i32 - 1);
    //         let bottom_right_char = get_char(matrix, x as i32 + 1, y as i32 + 1);
    //         let top_left_char = get_char(matrix, x as i32 - 1, y as i32 - 1);
    //         let top_right_char = get_char(matrix, x as i32 - 1, y as i32 + 1);
    //         if left_char.is_none() || left_char.unwrap() != self.char {
    //             if (top_char.is_none()
    //                 || (!visited_plots.contains(&(x - 1, y))
    //                     || top_left_char.is_some_and(|char| char == self.char)))
    //                 && (bottom_char.is_none()
    //                     || (!visited_plots.contains(&(x + 1, y))
    //                         || bottom_left_char.is_some_and(|char| char == self.char)))
    //             {
    //                 sides += 1;
    //             }
    //         }
    //         if right_char.is_none() || right_char.unwrap() != self.char {
    //             if (top_char.is_none()
    //                 || (!visited_plots.contains(&(x - 1, y))
    //                     || top_right_char.is_some_and(|char| char == self.char)))
    //                 && (bottom_char.is_none()
    //                     || (!visited_plots.contains(&(x + 1, y))
    //                         || bottom_right_char.is_some_and(|char| char == self.char)))
    //             {
    //                 sides += 1;
    //             }
    //         }
    //         if top_char.is_none() || top_char.unwrap() != self.char {
    //             if (left_char.is_none() || (!visited_plots.contains(&(x, y - 1)) || top_left_char.is_some_and(|char| char == self.char)))
    //                 && (right_char.is_none() || (!visited_plots.contains(&(x, y + 1)) || top_right_char.is_some_and(|char| char == self.char)))
    //             {
    //                 sides += 1;
    //             }
    //         }
    //         if bottom_char.is_none() || bottom_char.unwrap() != self.char {
    //             if (left_char.is_none() || (!visited_plots.contains(&(x, y - 1)) || bottom_left_char.is_some_and(|char| char == self.char)))
    //                 && (right_char.is_none() || (!visited_plots.contains(&(x, y + 1)) || bottom_right_char.is_some_and(|char| char == self.char)))
    //             {
    //                 sides += 1;
    //             }
    //         }
    //     }
    //     return sides as u32 * self.get_area() as u32;
    // }
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
struct Perimeter {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

fn get_char(matrix: &Vec<Vec<char>>, i: i32, j: i32) -> Option<char> {
    let height = matrix.len();
    let width = matrix[0].len();

    if i < 0 || i >= height as i32 {
        return None;
    }
    if j < 0 || j >= width as i32 {
        return None;
    }
    return Some(matrix[i as usize][j as usize]);
}
