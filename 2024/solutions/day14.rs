use std::collections::HashMap;

type Position = (usize, usize);
type Velocity = (isize, isize);

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[derive(Clone)]
pub struct Robot {
    pos: Position,
    vel: Velocity,
}

impl Robot {
    pub fn iterate(&mut self, n_times: usize) {
        (0..n_times).into_iter().for_each(|_i| self.next());
    }
    fn next(&mut self) {
        self.pos.0 = ((self.pos.0 as isize + self.vel.0).rem_euclid(HEIGHT as isize)) as usize;
        self.pos.1 = ((self.pos.1 as isize + self.vel.1).rem_euclid(WIDTH as isize)) as usize;
    }
}

pub fn parse<'p>(input_data: String) -> Vec<Robot> {
    input_data
        .lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<_>>();

            let pos = {
                let parts = parts[0].get(2..).unwrap().split(",").collect::<Vec<_>>();
                (parts[1].parse().unwrap(), parts[0].parse().unwrap())
            };
            let vel = {
                let parts = parts[1].get(2..).unwrap().split(",").collect::<Vec<_>>();
                (parts[1].parse().unwrap(), parts[0].parse().unwrap())
            };

            Robot { pos, vel }
        })
        .collect()
}

pub fn part_1(robots: &[Robot]) -> i64 {
    let mut robots = robots.to_vec();

    // Iter the robots
    robots.iter_mut().for_each(|r| {
        r.iterate(100);
    });

    // Check cuadrants
    let middles = (HEIGHT / 2, WIDTH / 2);
    let mut cuadrant_count: [usize; 4] = [0; 4];
    robots.iter().for_each(|r| {
        if r.pos.0 < middles.0 && r.pos.1 < middles.1 {
            // Top left
            cuadrant_count[0] += 1;
        } else if r.pos.0 < middles.0 && r.pos.1 > middles.1 {
            // Top right
            cuadrant_count[1] += 1;
        } else if r.pos.0 > middles.0 && r.pos.1 < middles.1 {
            // Bottom left
            cuadrant_count[2] += 1;
        } else if r.pos.0 > middles.0 && r.pos.1 > middles.1 {
            // Bottom right
            cuadrant_count[3] += 1;
        }
    });

    cuadrant_count.iter().product::<usize>() as i64
}

pub fn robots_to_ascii(robots: &[Robot]) -> Vec<Vec<char>> {
    let mut counts = HashMap::new();

    robots.iter().for_each(|r| {
        counts.entry(r.pos).and_modify(|c| *c += 1).or_insert(1);
    });

    (0..HEIGHT)
        .into_iter()
        .map(|i| {
            (0..WIDTH)
                .into_iter()
                .map(|j| {
                    let pos = (i, j);

                    if let Some(_) = counts.get(&pos) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect()
}

// At the beginning I solved this by just
// generating 10k images (yes) until I found
// the pattern manually, which is very easy
// to see.
//
// Now that I know it will be a filled tree,
// I find the max neighbour count.
pub fn part_2(robots: &[Robot]) -> i64 {
    let mut robots = robots.to_vec();

    const MAX_SECS: usize = 10000;
    let mut max_neighbour_count: Option<(usize, usize)> = None;

    (0..MAX_SECS).into_iter().for_each(|secs| {
        robots.iter_mut().for_each(|r| {
            r.iterate(1);
        });

        let positions = robots.iter().fold(HashMap::new(), |mut acc, r| {
            acc.entry(r.pos).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

        let cur_neighbour_count: usize = positions
            .iter()
            .map(|(pos, _)| {
                (-1..=1)
                    .into_iter()
                    .map(|d_i| {
                        (-1..=1)
                            .into_iter()
                            .filter_map(|d_j| {
                                if d_i != d_j {
                                    let i = pos.0 as isize + d_i;
                                    let j = pos.0 as isize + d_i;

                                    if i >= 0 && j >= 0 {
                                        let i = i as usize;
                                        let j = i as usize;

                                        if let Some(c) = positions.get(&(i, j)) {
                                            return Some(c);
                                        }
                                    }
                                }

                                None
                            })
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum();

        if let Some(ref mut max_neighbour_count) = max_neighbour_count {
            if cur_neighbour_count > max_neighbour_count.1 {
                *max_neighbour_count = (secs + 1, cur_neighbour_count);
            }
        } else {
            max_neighbour_count = Some((secs + 1, cur_neighbour_count));
        }
    });

    max_neighbour_count.unwrap().0 as i64
}
