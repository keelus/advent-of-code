use std::{collections::BinaryHeap, usize};

type Position = (usize, usize);

pub struct Input {
    map: Vec<Vec<char>>,
    start_pos: Position,
    end_pos: Position,
}

impl Input {
    pub fn new() -> Self {
        Self {
            map: vec![],
            start_pos: (0, 0),
            end_pos: (0, 0),
        }
    }
}

pub fn parse(input_data: String) -> Input {
    input_data
        .lines()
        .enumerate()
        .fold(Input::new(), |mut input_acc, (i, l)| {
            input_acc.map.push(
                l.chars()
                    .enumerate()
                    .fold(Vec::new(), |mut row_acc, (j, c)| {
                        match c {
                            'S' => {
                                input_acc.start_pos = (i, j);
                                row_acc.push('.');
                            }
                            'E' => {
                                input_acc.end_pos = (i, j);
                                row_acc.push('.');
                            }
                            _ => row_acc.push(c),
                        }

                        row_acc
                    }),
            );
            input_acc
        })
}

pub fn calculate_distances(
    start_pos: Position,
    end_pos: Position,
    map: &[Vec<char>],
) -> (Vec<Vec<usize>>, Vec<Position>) {
    let height = map.len();
    let width = map[0].len();

    #[derive(Copy, Clone)]
    struct State {
        visited: bool,
        predecessor: Option<Position>,
        dist: usize,
    }

    #[derive(Eq, PartialEq)]
    struct QueueEntry {
        dist: usize,
        pos: Position,
    }

    impl Ord for QueueEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.dist.cmp(&self.dist)
        }
    }

    impl PartialOrd for QueueEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut map_state = vec![
        vec![
            State {
                visited: false,
                predecessor: None,
                dist: usize::MAX,
            };
            width
        ];
        height
    ];

    let mut queue = BinaryHeap::new();

    map_state[start_pos.0][start_pos.1].dist = 0;
    queue.push(QueueEntry {
        dist: 0,
        pos: start_pos,
    });

    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        map_state[u.pos.0][u.pos.1].visited = true;

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for dir in dirs {
            let new_i = u.pos.0 as isize + dir.0;
            let new_j = u.pos.1 as isize + dir.1;

            if new_i >= 0 && new_j >= 0 && new_i < height as isize && new_j < width as isize {
                let v_pos = (new_i as usize, new_j as usize);

                if map[v_pos.0][v_pos.1] == '.' {
                    let v = &mut map_state[v_pos.0][v_pos.1];
                    let new_dist = u.dist + 1;

                    if v.dist > new_dist {
                        v.dist = new_dist;
                        v.predecessor = Some(u.pos);

                        queue.push(QueueEntry {
                            dist: u.dist + 1,
                            pos: v_pos,
                        });
                    }
                }
            }
        }
    }

    let mut path = Vec::new();

    let mut current = Some(end_pos);
    while current != None {
        let cur = current.unwrap();
        path.push(cur);
        current = map_state[cur.0][cur.1].predecessor;
    }
    path.reverse();

    (
        map_state
            .iter()
            .map(|row| row.iter().map(|elem| elem.dist).collect())
            .collect(),
        path,
    )
}

pub fn manhattan_distance(a: Position, b: Position) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

pub fn calculate_cheat_count<'a>(
    max_cheat: isize,
    min_saved: usize,
    map: &[Vec<char>],
    mut cur_pos: impl Iterator<Item = &'a Position>,
    distances: Vec<Vec<usize>>,
) -> usize {
    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut count = 0;
    while let Some(cur_pos) = cur_pos.next() {
        let base_cost = distances[cur_pos.0][cur_pos.1];

        for d_i in -max_cheat..=max_cheat {
            for d_j in -max_cheat..=max_cheat {
                if d_i == 0 && d_j == 0 {
                    continue;
                }

                let new_i = cur_pos.0 as isize + d_i;
                let new_j = cur_pos.1 as isize + d_j;

                if new_i >= 0 && new_i < height && new_j >= 0 && new_j < width {
                    let new_pos = (new_i as usize, new_j as usize);
                    if map[new_pos.0][new_pos.1] == '#' {
                        continue;
                    }

                    let dist_from_origin = manhattan_distance(*cur_pos, new_pos);

                    if dist_from_origin <= max_cheat as usize {
                        let new_cost = distances[new_pos.0][new_pos.1];

                        if new_cost > base_cost {
                            let saved = new_cost - base_cost - dist_from_origin;
                            if saved >= min_saved {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

pub fn part_1(input: &Input) -> i64 {
    let (distances, path) = calculate_distances(input.start_pos, input.end_pos, &input.map);
    calculate_cheat_count(2, 100, &input.map, path.iter(), distances) as i64
}

pub fn part_2(input: &Input) -> i64 {
    let (distances, path) = calculate_distances(input.start_pos, input.end_pos, &input.map);
    calculate_cheat_count(20, 100, &input.map, path.iter(), distances) as i64
}
