use std::collections::{HashMap, HashSet};

pub struct Map {
    antenna_map: HashMap<char, Vec<Position>>,
    height: isize,
    width: isize,
}

impl Map {
    fn is_inside(&self, pos: Position) -> bool {
        pos.0 >= 0 && pos.0 < self.height && pos.1 >= 0 && pos.1 < self.width
    }

    fn is_overlapping(&self, pos: Position) -> bool {
        self.antenna_map
            .iter()
            .any(|a| a.1.iter().any(|p| p.0 == pos.0 && p.1 == pos.1))
    }
}

type Position = (isize, isize);

pub fn parse(input_data: String) -> Map {
    let map_height = input_data.lines().count();
    let map_width = input_data.lines().next().unwrap().len();

    let antennas: Vec<_> = input_data
        .lines()
        .enumerate()
        .filter_map(|(i, l)| {
            let row = i.clone();
            let row_antennas = l
                .chars()
                .enumerate()
                .filter_map(move |(j, c)| (c != '.').then_some(((row as isize, j as isize), c)))
                .collect::<Vec<_>>();

            (!row_antennas.is_empty()).then_some(row_antennas)
        })
        .flatten()
        .collect();

    // Now, group the antenas
    let antennas: HashMap<char, Vec<Position>> =
        antennas.iter().fold(HashMap::new(), |mut acc, antenna| {
            acc.entry(antenna.1)
                .and_modify(|e| e.push(antenna.0))
                .or_insert(vec![antenna.0]);
            acc
        });

    Map {
        antenna_map: antennas,
        height: map_height as isize,
        width: map_width as isize,
    }
}

pub fn part_1(map: &Map) -> i64 {
    map.antenna_map
        .iter()
        .map(|(_k, positions)| {
            let len = positions.len();

            (0..len)
                .map(|i| {
                    let cur = positions[i];
                    (0..len)
                        .filter(|&j| {
                            if i == j {
                                return false;
                            }

                            let other = positions[j];
                            let (dx, dy) = (other.1 - cur.1, other.0 - cur.0);
                            let antinode_pos = (other.0 + dy, other.1 + dx);

                            if map.is_inside(antinode_pos) {
                                if !map.is_overlapping(antinode_pos) {
                                    return true;
                                }
                            }

                            false
                        })
                        .count() as i64
                })
                .sum::<i64>()
        })
        .sum()
}

pub fn part_2(map: &Map) -> i64 {
    map.antenna_map
        .iter()
        .fold(HashSet::new(), |mut acc, (_k, positions)| {
            let len = positions.len();

            (0..len).for_each(|i| {
                let cur = positions[i];
                (0..len).for_each(|j| {
                    if i != j {
                        let other = positions[j];
                        let (dx, dy) = (other.1 - cur.1, other.0 - cur.0);
                        let mut antinode_pos = (cur.0, cur.1);

                        while map.is_inside(antinode_pos) {
                            acc.insert(antinode_pos);

                            antinode_pos.0 += dy;
                            antinode_pos.1 += dx;
                        }
                    }
                });
            });

            acc
        })
        .len() as i64
}
