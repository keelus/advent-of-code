use std::{
    collections::{HashMap, HashSet},
    usize,
};

type Position = (usize, usize);

#[derive(Debug)]
pub struct Region {
    w: usize,
    h: usize,
    points: HashSet<Position>,
}

pub struct Side {
    from: Position,
    to: Position,
}

impl Side {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct ComplexSide {
    from: Position,
    to_horiz: Option<Position>,
    to_vert: Option<Position>,
}

impl ComplexSide {
    fn map_from_side_vec(sides: &[Side]) -> HashMap<Position, Self> {
        let mut map: HashMap<Position, Self> = HashMap::new();

        sides.iter().for_each(|side| {
            let from = side.from;
            let to = side.to;

            let mut new_horiz = None;
            let mut new_vert = None;

            if from.0 == to.0 {
                new_horiz = Some(to);
            } else {
                new_vert = Some(to);
            }

            if let Some(ref mut entry) = map.get_mut(&from) {
                if new_horiz.is_some() {
                    entry.to_vert = new_horiz;
                }

                if new_vert.is_some() {
                    entry.to_vert = new_vert;
                }
            } else {
                map.insert(
                    from,
                    ComplexSide {
                        from,
                        to_horiz: new_horiz,
                        to_vert: new_vert,
                    },
                );
            }
        });

        map
    }
}

impl Region {
    pub fn get_area(&self) -> usize {
        self.points.len()
    }

    pub fn get_sides(&self) -> Vec<Side> {
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
        let mut sides_new: Vec<Side> = Vec::new();
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

                    let is_outside = i < 0 || i >= data.len() as isize || j < 0 || j >= w as isize;

                    if is_outside || !data[i as usize][j as usize] {
                        let from;
                        let to;

                        match dir {
                            (-1, 0) => {
                                from = pos;
                                to = (pos.0, pos.1 + 1);
                            }
                            (1, 0) => {
                                from = (pos.0 + 1, pos.1);
                                to = (pos.0 + 1, pos.1 + 1);
                            }
                            (0, -1) => {
                                from = pos;
                                to = (pos.0 + 1, pos.1);
                            }
                            (0, 1) => {
                                from = (pos.0, pos.1 + 1);
                                to = (pos.0 + 1, pos.1 + 1);
                            }
                            _ => unreachable!(),
                        }

                        sides_new.push(Side::new(from, to));
                    }
                }
            }
        }
        sides_new
    }

    pub fn get_perimeter(&self) -> usize {
        self.get_sides().len()
    }

    pub fn merge_sides(&self) -> Vec<Side> {
        let sides = ComplexSide::map_from_side_vec(&self.get_sides());

        // (vert, horiz)
        let mut used_sides = (HashSet::new(), HashSet::new());
        let mut point_count = HashMap::new();

        // A first pass to:
        // 1. Count how many sides start and end at each point.
        // 2. Get the sides that will be used for merging (the ones inside other sides).
        sides.iter().for_each(|(_, cpx_side)| {
            let cur_from = cpx_side.from;

            point_count
                .entry(cur_from)
                .and_modify(|c| *c += 1)
                .or_insert(1);

            // Do horizontals
            let final_to = cpx_side.to_horiz;
            if final_to.is_some() {
                let mut final_to = final_to.unwrap();

                point_count
                    .entry(final_to)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_horiz) = other_cpx_side.to_horiz {
                            // Is Expandable
                            used_sides.1.insert(other_cpx_side.from);
                            final_to = other_to_horiz;
                            continue;
                        }
                    }

                    // Not expandable
                    break;
                }
            }

            // Do verticals
            let final_to = cpx_side.to_vert;
            if final_to.is_some() {
                let mut final_to = final_to.unwrap();

                point_count
                    .entry(final_to)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_vert) = other_cpx_side.to_vert {
                            // Is Expandable
                            final_to = other_to_vert;
                            used_sides.0.insert(other_cpx_side.from);
                            continue;
                        }
                    }

                    // Not expandable.
                    break;
                }
            }
        });

        // Allow those points with greater count to
        // be merged from, as they wont be consumed
        // from bigger sides.
        point_count.iter().for_each(|(p, c)| {
            if *c > 2 {
                used_sides.0.remove(p);
                used_sides.1.remove(p);
            }
        });

        // Second pass
        sides
            .iter()
            .map(|(_, cpx_side)| {
                let mut cur_merged = Vec::with_capacity(2);
                let cur_from = cpx_side.from;

                // Do horizontals
                let final_to = cpx_side.to_horiz;
                if !used_sides.1.contains(&cur_from) && final_to.is_some() {
                    let mut final_to = final_to.unwrap();
                    loop {
                        if let Some(other_cpx_side) = sides.get(&final_to) {
                            if let Some(other_to_horiz) = other_cpx_side.to_horiz {
                                if let Some(&count) = point_count.get(&other_to_horiz) {
                                    // Is Expandable
                                    final_to = other_to_horiz;

                                    // If is further expandable
                                    if count <= 2 {
                                        continue;
                                    }
                                }
                            }
                        }
                        break;
                    }

                    cur_merged.push(Side {
                        from: cur_from,
                        to: final_to,
                    });
                }

                // Do verticals
                let final_to = cpx_side.to_vert;
                if !used_sides.0.contains(&cur_from) && final_to.is_some() {
                    let mut final_to = final_to.unwrap();
                    loop {
                        if let Some(other_cpx_side) = sides.get(&final_to) {
                            if let Some(other_to_vert) = other_cpx_side.to_vert {
                                if let Some(&count) = point_count.get(&other_to_vert) {
                                    // Is Expandable
                                    final_to = other_to_vert;

                                    // If is further expandable
                                    if count <= 2 {
                                        continue;
                                    }
                                }
                            }
                        }
                        break;
                    }

                    // Not expandable, save it
                    cur_merged.push(Side {
                        from: cur_from,
                        to: final_to,
                    });
                }

                cur_merged
            })
            .flatten()
            .collect()
    }

    fn calc_fence_price(&self) -> usize {
        self.get_area() * self.get_perimeter()
    }

    fn calc_fence_price_p2(&self) -> usize {
        self.get_area() * self.merge_sides().len()
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
    let mut min = (usize::MAX, usize::MAX);
    let mut max = (usize::MIN, usize::MIN);

    visited.iter().for_each(|&(i, j)| {
        min.0 = min.0.min(i);
        max.0 = max.0.max(i);
        min.1 = min.1.min(j);
        max.1 = max.1.max(j);
    });

    let max_i = max.0 - min.0;
    let max_j = max.1 - min.1;

    let visited = visited
        .iter()
        .map(|(i, j)| (i - min.0, j - min.1))
        .collect::<HashSet<_>>();

    Some(Region {
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
            sum += region.calc_fence_price()
        } else {
            break;
        }
    });

    sum as i64
}

pub fn part_2(input: &Input) -> i64 {
    let mut map = input.map.to_vec();

    let mut sum = 0;
    input.plant_types.iter().for_each(|&p| loop {
        if let Some(region) = get_region(p, &mut map) {
            sum += region.calc_fence_price_p2()
        } else {
            break;
        }
    });

    sum as i64
}
