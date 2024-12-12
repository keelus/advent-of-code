use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

type Position = (usize, usize);

#[derive(Debug)]
pub struct Region {
    symbol: u8,
    w: usize,
    h: usize,
    points: HashSet<Position>,
}

#[derive(Eq, Hash, PartialEq)]
pub struct ComplexSide {
    from: Position,
    to_horiz: Option<Position>,
    to_vert: Option<Position>,
}

impl Debug for ComplexSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.to_horiz.is_some() && self.to_vert.is_some() {
            write!(
                f,
                "{:?}->[h:{:?}, v:{:?}]",
                self.from,
                self.to_horiz.unwrap(),
                self.to_vert.unwrap()
            )
        } else if self.to_horiz.is_some() {
            write!(f, "{:?}->h:{:?}", self.from, self.to_horiz.unwrap())
        } else if self.to_vert.is_some() {
            write!(f, "{:?}->v:{:?}", self.from, self.to_vert.unwrap())
        } else {
            write!(f, "{:?}->X", self.from)
        }
    }
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

pub struct Side {
    from: Position,
    to: Position,
}

impl Debug for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}->{:?}", self.from, self.to)
    }
}

impl Side {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
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

    // Works correctly, except that puzzle-mentioned
    // Moebious specific case.
    pub fn merge_sides(&self) -> Vec<Side> {
        let sides = ComplexSide::map_from_side_vec(&self.get_sides());

        let mut horiz_ign = HashSet::new();
        let mut vert_ign = HashSet::new();

        // First pass:
        sides.iter().for_each(|(_, cpx_side)| {
            let cur_from = cpx_side.from;

            // Do horizontals
            let final_to = cpx_side.to_horiz;
            if final_to.is_some() {
                let mut final_to = final_to.unwrap();
                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_horiz) = other_cpx_side.to_horiz {
                            horiz_ign.insert(other_cpx_side.from);
                            // Is Expandable
                            final_to = other_to_horiz;
                            // viewed_horiz.insert(other_cpx_side.from);
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
                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_vert) = other_cpx_side.to_vert {
                            // Is Expandable
                            final_to = other_to_vert;
                            vert_ign.insert(other_cpx_side.from);
                            continue;
                        }
                    }

                    // Not expandable.
                    break;
                }
            }
        });

        let mut merged_sides: Vec<Side> = Vec::new();

        // Second pass
        sides.iter().for_each(|(_, cpx_side)| {
            let cur_from = cpx_side.from;

            // Do horizontals
            let final_to = cpx_side.to_horiz;
            if !horiz_ign.contains(&cur_from) && final_to.is_some() {
                let mut final_to = final_to.unwrap();
                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_horiz) = other_cpx_side.to_horiz {
                            horiz_ign.insert(other_cpx_side.from);
                            // Is Expandable
                            final_to = other_to_horiz;
                            continue;
                        }
                    }

                    // Not expandable, save it
                    merged_sides.push(Side {
                        from: cur_from,
                        to: final_to,
                    });
                    break;
                }
            }

            // Do verticals
            let final_to = cpx_side.to_vert;
            if !vert_ign.contains(&cur_from) && final_to.is_some() {
                let mut final_to = final_to.unwrap();
                loop {
                    if let Some(other_cpx_side) = sides.get(&final_to) {
                        if let Some(other_to_vert) = other_cpx_side.to_vert {
                            // Is Expandable
                            final_to = other_to_vert;
                            vert_ign.insert(other_cpx_side.from);
                            continue;
                        }
                    }

                    // Not expandable, save it
                    merged_sides.push(Side {
                        from: cur_from,
                        to: final_to,
                    });
                    break;
                }
            }
        });

        merged_sides
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
