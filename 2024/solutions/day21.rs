use std::collections::HashSet;

use itertools::Itertools;

type Position = (usize, usize);

pub trait Keypad {
    fn get_position_from_key(key: char) -> Position;
    fn get_key_from_position(pos: Position) -> char;

    fn width() -> isize;
    fn height() -> isize;

    fn shortest_paths(
        current_key: char,
        target_key: char,
        visited: &mut HashSet<Position>,
        mut current_path: Vec<Position>,
    ) -> Vec<Vec<Position>> {
        if current_key == ' ' {
            return vec![];
        }

        let cur_pos = Self::get_position_from_key(current_key);
        current_path.push(cur_pos);

        if current_key == target_key {
            return vec![current_path];
        }

        if visited.contains(&cur_pos) {
            return vec![];
        }

        visited.insert(cur_pos);

        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut new_paths = Vec::new();
        for dir in dirs {
            let new_i = cur_pos.0 as isize + dir.0;
            let new_j = cur_pos.1 as isize + dir.1;

            if new_i >= 0 && new_i < Self::height() && new_j >= 0 && new_j < Self::width() {
                let new_pos = (new_i as usize, new_j as usize);
                let cur_key = Self::get_key_from_position(new_pos);

                let gotten_paths = Self::shortest_paths(
                    cur_key,
                    target_key,
                    &mut visited.clone(),
                    current_path.clone(),
                );

                for gotten_path in gotten_paths {
                    new_paths.push(gotten_path);
                }
            }
        }

        if new_paths.is_empty() {
            return vec![];
        } else {
            let shortest_len = new_paths.iter().map(|p| p.len()).min().unwrap();
            new_paths
                .into_iter()
                .filter(|p| p.len() == shortest_len)
                .collect()
        }
    }

    fn calculate_direction_strings(target_codes: Vec<String>) -> Vec<String> {
        let directions = target_codes
            .iter()
            .map(|target_code| {
                let mut current_key = 'A';
                let directions = target_code
                    .chars()
                    .map(|c| {
                        let shortest_paths =
                            Self::shortest_paths(current_key, c, &mut HashSet::new(), vec![]);
                        let directions = shortest_paths
                            .iter()
                            .map(|path| path_to_directions(path.to_vec()))
                            .collect::<Vec<_>>();

                        current_key = c;
                        directions
                    })
                    .collect::<Vec<_>>();

                directions
                    .iter()
                    .map(|v| v.iter())
                    .multi_cartesian_product()
                    .map(|dirs| dir_vec_to_str(dirs))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();

        let shortest_len = directions.iter().map(|d| d.len()).min().unwrap();
        directions
            .into_iter()
            .filter(|d| d.len() == shortest_len)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<String>>()
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
pub struct NumericKeypad();

impl Keypad for NumericKeypad {
    fn height() -> isize {
        4
    }

    fn width() -> isize {
        3
    }

    fn get_position_from_key(key: char) -> Position {
        match key {
            '7' => (0, 0),
            '8' => (0, 1),
            '9' => (0, 2),
            '4' => (1, 0),
            '5' => (1, 1),
            '6' => (1, 2),
            '1' => (2, 0),
            '2' => (2, 1),
            '3' => (2, 2),
            ' ' => (3, 0),
            '0' => (3, 1),
            'A' => (3, 2),
            _ => unreachable!(),
        }
    }

    fn get_key_from_position(pos: Position) -> char {
        match pos {
            (0, 0) => '7',
            (0, 1) => '8',
            (0, 2) => '9',
            (1, 0) => '4',
            (1, 1) => '5',
            (1, 2) => '6',
            (2, 0) => '1',
            (2, 1) => '2',
            (2, 2) => '3',
            (3, 0) => ' ',
            (3, 1) => '0',
            (3, 2) => 'A',
            _ => unreachable!(),
        }
    }
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
pub struct DirectionalKeypad();

impl Keypad for DirectionalKeypad {
    fn height() -> isize {
        2
    }

    fn width() -> isize {
        3
    }

    fn get_position_from_key(key: char) -> Position {
        match key {
            ' ' => (0, 0),
            '^' => (0, 1),
            'A' => (0, 2),
            '<' => (1, 0),
            'v' => (1, 1),
            '>' => (1, 2),
            _ => unreachable!(),
        }
    }

    fn get_key_from_position(pos: Position) -> char {
        match pos {
            (0, 0) => ' ',
            (0, 1) => '^',
            (0, 2) => 'A',
            (1, 0) => '<',
            (1, 1) => 'v',
            (1, 2) => '>',
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse(input_data: String) -> Vec<String> {
    input_data.lines().map(|l| l.to_string()).collect()
}

pub fn dir_vec_to_str(dirs: Vec<&Vec<Direction>>) -> String {
    dirs.iter()
        .map(|dirs| {
            dirs.iter()
                .map(|dir| {
                    let key_byte = match dir {
                        Direction::Left => '<',
                        Direction::Right => '>',
                        Direction::Up => '^',
                        Direction::Down => 'v',
                    };
                    key_byte
                })
                .chain(std::iter::once('A'))
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect::<String>()
}

pub fn path_to_directions(path: Vec<Position>) -> Vec<Direction> {
    path.windows(2)
        .map(|w| {
            let a = w[0];
            let b = w[1];

            let d_i = b.0 as isize - a.0 as isize;
            let d_j = b.1 as isize - a.1 as isize;

            if d_i == -1 {
                Direction::Up
            } else if d_i == 1 {
                Direction::Down
            } else if d_j == -1 {
                Direction::Left
            } else if d_j == 1 {
                Direction::Right
            } else {
                unreachable!("Got diffs of d_i={}, d_j={}", d_i, d_j);
            }
        })
        .collect()
}

pub fn numeric_part_of_code(code: &str) -> usize {
    code.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

pub fn part_1(codes: &[String]) -> i64 {
    fn min_dirs_len_of_code(code: &str) -> usize {
        let keypad_directions = NumericKeypad::calculate_direction_strings(vec![code.to_string()]);
        let dirpad_directions = DirectionalKeypad::calculate_direction_strings(keypad_directions);
        let dirpad_two_dirs = DirectionalKeypad::calculate_direction_strings(dirpad_directions);

        dirpad_two_dirs[0].len()
    }

    codes
        .iter()
        .map(|code| {
            let min_len = min_dirs_len_of_code(code);
            let numeric_part = numeric_part_of_code(code);
            min_len * numeric_part
        })
        .sum::<usize>() as i64
}

pub fn part_2(_input: &[String]) -> i64 {
    0
}
