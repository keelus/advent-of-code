use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

const MAP_SIZE: usize = 71;

pub fn parse(input_data: String) -> Vec<Position> {
    input_data
        .lines()
        .map(|l| {
            let l = l.trim().split(",").collect::<Vec<_>>();
            let i = l[1].parse().unwrap();
            let j = l[0].parse().unwrap();
            (i, j)
        })
        .collect()
}

pub fn get_min_steps(
    memory: &HashSet<Position>,
    start_pos: Position,
    end_pos: Position,
) -> Option<usize> {
    let mut predecessor = HashMap::new();
    let mut visited = HashSet::new();
    let mut distance = HashMap::new();

    struct State {
        dist: usize,
        pos: Position,
    }

    let mut queue = Vec::new();
    queue.push(State {
        dist: 0,
        pos: start_pos,
    });

    while !queue.is_empty() {
        let u = {
            let idx = queue
                .iter()
                .enumerate()
                .min_by_key(|(_, elem)| elem.dist)
                .map(|(i, _)| i)
                .unwrap();

            queue.remove(idx)
        };
        visited.insert(u.pos);

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for dir in dirs {
            let new_i = u.pos.0 as isize + dir.0;
            let new_j = u.pos.1 as isize + dir.1;

            if new_i >= 0 && new_j >= 0 && new_i < MAP_SIZE as isize && new_j < MAP_SIZE as isize {
                let v_pos = (new_i as usize, new_j as usize);

                if !memory.contains(&v_pos) {
                    let v_dist = *distance.get(&v_pos).unwrap_or(&usize::MAX);

                    if v_dist > u.dist + 1 {
                        let v = State {
                            dist: u.dist + 1,
                            pos: v_pos,
                        };

                        distance.insert(v.pos, v.dist);
                        predecessor.insert(v.pos, u.pos);
                        queue.push(v);
                    }
                }
            }
        }
    }

    distance.get(&end_pos).copied()
}

pub fn part_1(falling_bytes: &[Position]) -> i64 {
    let memory_map = falling_bytes
        .get(..1024)
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    if let Some(distance) = get_min_steps(&memory_map, (0, 0), (MAP_SIZE - 1, MAP_SIZE - 1)) {
        distance as i64
    } else {
        println!("No path found!");
        0
    }
}

pub fn part_2(falling_bytes: &[Position]) -> i64 {
    let mut memory_map = HashSet::new();

    let mut blocked = false;
    let mut falling_idx = 0;
    loop {
        memory_map.insert(falling_bytes[falling_idx]);
        let min_path = get_min_steps(&memory_map, (0, 0), (MAP_SIZE - 1, MAP_SIZE - 1));

        if min_path.is_none() {
            blocked = true;
            break;
        }

        falling_idx += 1;
        if falling_idx >= falling_bytes.len() {
            break;
        }
    }

    if blocked {
        let byte = falling_bytes.get(falling_idx).unwrap();
        println!("Part 2: ({},{})", byte.1, byte.0);
    }

    0
}
