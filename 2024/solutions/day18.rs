use std::collections::BinaryHeap;

#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    Blocked,
}

type Position = (usize, usize);
type Map = [[Cell; MAP_SIZE]; MAP_SIZE];

pub struct Input {
    map: Map,
    falling_blocks: Vec<Position>,
}

const MAP_SIZE: usize = 71;

pub fn parse(input_data: String) -> Input {
    let map = [[Cell::Empty; MAP_SIZE]; MAP_SIZE];

    let falling_blocks = input_data
        .lines()
        .map(|l| {
            let l = l.trim().split(",").collect::<Vec<_>>();
            let i = l[1].parse().unwrap();
            let j = l[0].parse().unwrap();
            (i, j)
        })
        .collect();

    Input {
        map,
        falling_blocks,
    }
}

pub fn get_min_steps(memory: &Map, start_pos: Position, end_pos: Position) -> usize {
    #[derive(Copy, Clone)]
    struct State {
        visited: bool,
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

    let mut map_state = [[State {
        visited: false,
        dist: usize::MAX,
    }; MAP_SIZE]; MAP_SIZE];

    let mut queue = BinaryHeap::new();

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

            if new_i >= 0 && new_j >= 0 && new_i < MAP_SIZE as isize && new_j < MAP_SIZE as isize {
                let v_pos = (new_i as usize, new_j as usize);

                if memory[v_pos.0][v_pos.1] == Cell::Empty {
                    let v = &mut map_state[v_pos.0][v_pos.1];
                    let new_dist = u.dist + 1;

                    if v.dist > new_dist {
                        v.dist = new_dist;

                        queue.push(QueueEntry {
                            dist: u.dist + 1,
                            pos: v_pos,
                        });
                    }
                }
            }
        }
    }

    map_state[end_pos.0][end_pos.1].dist
}

pub fn part_1(input: &Input) -> i64 {
    let mut map = input.map.clone();

    input
        .falling_blocks
        .get(..1024)
        .unwrap()
        .iter()
        .for_each(|pos| {
            map[pos.0][pos.1] = Cell::Blocked;
        });

    get_min_steps(&map, (0, 0), (MAP_SIZE - 1, MAP_SIZE - 1)) as i64
}

pub fn part_2(input: &Input) -> i64 {
    let mut map = input.map.clone();

    let mut blocked = false;
    let mut falling_idx = 0;
    loop {
        let falling_block = input.falling_blocks[falling_idx];
        map[falling_block.0][falling_block.1] = Cell::Blocked;
        let min_distance = get_min_steps(&map, (0, 0), (MAP_SIZE - 1, MAP_SIZE - 1));

        if min_distance == usize::MAX {
            blocked = true;
            break;
        }

        falling_idx += 1;
        if falling_idx >= input.falling_blocks.len() {
            break;
        }
    }

    if blocked {
        let byte = input.falling_blocks.get(falling_idx).unwrap();
        println!("Part 2: ({},{})", byte.1, byte.0);
    }

    0
}
