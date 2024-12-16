use std::collections::HashMap;

type Position = (usize, usize);

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
    pub fn turn_90_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_90_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    pub fn move_position_towards(&self, pos: Position) -> Position {
        let delta = match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };

        let new_i = pos.0 as isize + delta.0;
        let new_j = pos.1 as isize + delta.1;

        if new_i >= 0 && new_j >= 0 {
            return (new_i as usize, new_j as usize);
        }

        unreachable!("Border should never be crossed.");
    }
}

pub struct Input {
    map: Vec<Vec<u8>>,
    start_position: Position,
    end_position: Position,
}

pub fn parse(input_data: String) -> Input {
    let mut start_position = (0, 0);
    let mut end_position = (0, 0);

    let map = input_data
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.bytes()
                .enumerate()
                .map(|(j, b)| {
                    if b == b'S' {
                        start_position = (i, j);
                        b'.'
                    } else if b == b'E' {
                        end_position = (i, j);
                        b'.'
                    } else {
                        b
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Input {
        map,
        start_position,
        end_position,
    }
}

pub fn explore(
    pos: Position,
    dir: Direction,
    accum_points: usize,
    input: &Input,
    visited: &mut HashMap<(Position, Direction), usize>,
) {
    if input.map[pos.0][pos.1] == b'#' {
        return;
    }

    let min_accum_points = *visited.get(&(pos, dir)).unwrap_or(&usize::MAX);
    if accum_points > min_accum_points {
        return;
    }

    visited.insert((pos, dir), accum_points);

    if pos == input.end_position {
        return;
    }

    let new_dirs = [dir, dir.turn_90_left(), dir.turn_90_right()];
    new_dirs.into_iter().for_each(|new_dir| {
        let accum_points = accum_points + if new_dir == dir { 1 } else { 1001 };
        let new_pos = new_dir.move_position_towards(pos);
        explore(new_pos, new_dir, accum_points, input, visited);
    });
}

pub fn part_1(input: &Input) -> i64 {
    let mut visited = HashMap::new();
    explore(
        input.start_position,
        Direction::Right,
        0,
        &input,
        &mut visited,
    );

    *Direction::all()
        .into_iter()
        .filter_map(|dir| visited.get(&(input.end_position, dir)))
        .min()
        .unwrap() as i64
}

pub fn part_2(input: &Input) -> i64 {
    0
}
