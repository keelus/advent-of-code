use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right_90(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

type Position = (usize, usize);

pub struct Input {
    map: Vec<Vec<char>>,
    start_pos: Position,
}

fn move_towards(pos: Position, dir: Direction) -> Position {
    match dir {
        // Allow overflows, as they will make the guard
        // exit the map anyways.
        Direction::Up => (pos.0.wrapping_sub(1), pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1.wrapping_sub(1)),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn is_outside(pos: Position, map: &[Vec<char>]) -> bool {
    let (height, width) = (map.len(), map[0].len());

    pos.0 >= height || pos.1 >= width
}

fn collides(pos: Position, map: &[Vec<char>]) -> bool {
    map.get(pos.0 as usize)
        .and_then(|row| row.get(pos.1 as usize))
        .is_some_and(|&c| c == '#')
}

pub fn parse(input_data: String) -> Input {
    let start_pos = input_data
        .lines()
        .enumerate()
        .fold(None, |start_pos, (i, l)| {
            if start_pos.is_some() {
                return start_pos;
            }

            l.chars()
                .enumerate()
                .find(|(_, c)| *c == '^')
                .map(|(j, _)| (i, j))
                .or(start_pos)
        })
        .expect("Starting position not found for the given input.");

    let map = input_data.lines().map(|l| l.chars().collect()).collect();

    Input { map, start_pos }
}

fn solve_path(input: &Input) -> HashSet<Position> {
    let mut pos = input.start_pos;
    let mut dir = Direction::Up;

    let mut seen = HashSet::new();
    seen.insert(pos);

    loop {
        let new_pos = move_towards(pos, dir);

        if is_outside(new_pos, &input.map) {
            break;
        }

        if collides(new_pos, &input.map) {
            dir = dir.rotate_right_90();
        } else {
            pos = new_pos;
            seen.insert(pos);
        }
    }

    seen
}

pub fn part_1(input: &Input) -> i64 {
    solve_path(input).len() as i64
}

pub fn part_2(input: &Input) -> i64 {
    // Extract the base path, just exactly as
    // we did in part 1.
    let base_path = solve_path(input);

    // Then, bruteforce obstacles one by
    // one along the path.
    base_path
        .iter()
        .fold(HashSet::new(), |mut acc, &(i, j)| {
            let new_obstacle_pos = (i, j);
            if input.start_pos == new_obstacle_pos || input.map[i][j] == '#' {
                return acc;
            }

            let mut pos = input.start_pos;
            let mut dir = Direction::Up;

            let mut seen: HashSet<(Position, Direction)> = HashSet::new();
            seen.insert((pos, dir));

            // Now, it's just like part 1,
            // but detecting loops.
            loop {
                let new_pos = move_towards(pos, dir);

                if is_outside(new_pos, &input.map) {
                    break;
                }

                if new_pos == new_obstacle_pos || collides(new_pos, &input.map) {
                    dir = dir.rotate_right_90();
                } else {
                    pos = new_pos;

                    if seen.contains(&(pos, dir)) {
                        // Is a loop
                        acc.insert((i, j));
                        break;
                    } else {
                        seen.insert((pos, dir));
                    }
                }
            }

            acc
        })
        .len() as i64
}
