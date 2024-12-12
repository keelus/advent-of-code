use std::collections::HashSet;

type Position = (usize, usize);

#[derive(Debug)]
pub struct Region {
    symbol: u8,
    w: usize,
    h: usize,
    points: HashSet<Position>,
}

fn print_shape(shape: Vec<Vec<bool>>) {
    shape.iter().for_each(|row| {
        row.iter()
            .for_each(|&s| if s { print!("#") } else { print!(".") });
        println!("");
    })
}

impl Region {
    pub fn get_area(&self) -> usize {
        self.points.len()
    }

    pub fn get_perimeter(&self) -> usize {
        // Generate a map of true/false
        let data: Vec<Vec<bool>> = (0..self.h)
            .into_iter()
            .map(|i| {
                (0..self.w)
                    .into_iter()
                    .map(|j| self.points.contains(&(i, j)))
                    .collect()
            })
            .collect();

        // Now that we have the "shape"
        let mut perimeter = 0;
        for i in 0..data.len() {
            let w = data[i].len();

            for j in 0..data[i].len() {
                let b = data[i][j];
                if !b {
                    continue;
                }

                let pos = (i, j);

                let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

                for dir in dirs {
                    let i = pos.0 as isize + dir.0 as isize;
                    let j = pos.1 as isize + dir.1 as isize;

                    if i < 0 || i >= data.len() as isize || j < 0 || j >= w as isize {
                        perimeter += 1;
                        continue;
                    }

                    let i = i as usize;
                    let j = j as usize;

                    if !data[i][j] {
                        // There is no at that side
                        perimeter += 1;
                    }
                }
            }
        }

        // print_shape(data);
        perimeter
    }

    fn calc_fence_price(&self) -> usize {
        self.get_area() * self.get_perimeter()
    }
}

pub struct Input {
    plant_types: Vec<u8>,
    map: Vec<Vec<u8>>,
}

pub fn parse<'p>(input_data: String) -> Input {
    let plant_types = {
        let mut plant_types = HashSet::new();
        input_data.lines().for_each(|l| {
            l.bytes().for_each(|c| {
                plant_types.insert(c);
            })
        });

        plant_types.into_iter().collect::<Vec<_>>()
    };

    let map = input_data.lines().map(|l| l.bytes().collect()).collect();

    Input { plant_types, map }
}

fn explore(
    target: u8,
    dim: Position,
    pos: Position,
    map: &[Vec<u8>],
    visited: &mut HashSet<Position>,
) -> Vec<Position> {
    if visited.contains(&pos) {
        return vec![];
    }

    visited.insert(pos);

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut next_positions = Vec::new();
    for dir in dirs {
        let i = pos.0 as isize + dir.0 as isize;
        let j = pos.1 as isize + dir.1 as isize;

        if i < 0 || i >= dim.0 as isize || j < 0 || j >= dim.1 as isize {
            continue;
        }

        let i = i as usize;
        let j = j as usize;

        let b = map[i][j];

        if b != target {
            continue;
        }

        // If is target, keep exploring

        for p in explore(target, dim, (i, j), map, visited) {
            next_positions.push(p);
        }
    }

    next_positions
}

fn get_region(target: u8, map: &mut [Vec<u8>]) -> Option<Region> {
    let h = map.len();
    let w = map[0].len();

    // Find first A ever
    let mut coord: Option<Position> = None;
    'outer: for (i, line) in map.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == target {
                coord = Some((i, j));
                break 'outer;
            }
        }
    }

    let coord = if coord.is_none() {
        return None;
    } else {
        coord.unwrap()
    };

    let mut visited = HashSet::new();

    let _ = explore(target, (h, w), coord, map, &mut visited);

    // Now for each visited, remove from the original map
    visited.iter().for_each(|pos| {
        map[pos.0][pos.1] = b'#';
    });

    let visited = visited.into_iter().collect::<Vec<_>>();
    let mut min_i = 99;
    let mut min_j = 99;
    let mut max_i = 0;
    let mut max_j = 0;
    visited.iter().for_each(|&(i, j)| {
        if i < min_i {
            min_i = i;
        }
        if i > max_i {
            max_i = i;
        }

        if j < min_j {
            min_j = j;
        }

        if j > max_j {
            max_j = j;
        }
    });

    let max_i = max_i - min_i;
    let max_j = max_j - min_j;

    let visited = visited
        .iter()
        .map(|(i, j)| (i - min_i, j - min_j))
        .collect::<HashSet<_>>();

    // We can now construct the current region.
    Some(Region {
        symbol: target,
        w: max_j + 1,
        h: max_i + 1,
        points: visited,
    })
}

pub fn part_1(input: &Input) -> i64 {
    let mut map = input.map.to_vec();

    let mut sum = 0;
    input.plant_types.iter().for_each(|&p| loop {
        if let Some(region) = get_region(p, &mut map) {
            // println!(
            //     "Region {}:\n\tArea: {}\n\tPerimeter: {}\n\tFence price: {}",
            //     region.symbol as char,
            //     region.get_area(),
            //     region.get_perimeter(),
            //     region.calc_fence_price()
            // );
            sum += region.calc_fence_price()
        } else {
            break;
        }
    });

    sum as i64
}

pub fn part_2(input: &Input) -> i64 {
    0
}
