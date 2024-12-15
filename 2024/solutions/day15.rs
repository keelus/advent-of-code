type Position = (usize, usize);

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse_vec_from_str<'a, T>(direction_lines: T) -> Vec<Direction>
    where
        T: Iterator<Item = &'a str>,
    {
        direction_lines
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
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
    warehouse_1: part_1::Warehouse,
    warehouse_2: part_2::Warehouse,
    directions: Vec<Direction>,
}

pub fn parse<'p>(input_data: String) -> Input {
    Input {
        warehouse_1: part_1::parse(&input_data),
        warehouse_2: part_2::parse(&input_data),
        directions: Direction::parse_vec_from_str(
            input_data.lines().skip_while(|l| !l.is_empty()).skip(1),
        ),
    }
}

mod part_1 {
    use super::{Direction, Position};
    use std::collections::HashSet;

    #[derive(Clone)]
    pub struct Warehouse {
        robot: Position,
        walls: HashSet<Position>,
        boxes: HashSet<Position>,
    }

    impl Warehouse {
        pub fn new() -> Self {
            Self {
                robot: (0, 0),
                walls: HashSet::new(),
                boxes: HashSet::new(),
            }
        }

        pub fn move_robot(&mut self, dir: &Direction) {
            let new_pos = dir.move_position_towards(self.robot);

            if self.walls.contains(&new_pos) {
                // Against a wall, do nothing
                return;
            }

            if self.boxes.contains(&new_pos) {
                // Against a box, try propagating
                if !self.move_box(new_pos, dir) {
                    return;
                }
            }

            self.robot = new_pos;
        }

        fn move_box(&mut self, pos: Position, dir: &Direction) -> bool {
            if self.walls.contains(&pos) {
                return false;
            }

            if !self.boxes.contains(&pos) {
                true
            } else {
                let new_pos = dir.move_position_towards(pos);
                let next_moved = self.move_box(new_pos, dir);

                if next_moved {
                    self.boxes.remove(&pos);
                    self.boxes.insert(new_pos);
                }

                next_moved
            }
        }

        pub fn get_sum(&self) -> usize {
            self.boxes.iter().map(|pos| pos.0 * 100 + pos.1).sum()
        }
    }

    pub fn parse(input_data: &str) -> Warehouse {
        let warehouse = input_data
            .lines()
            .enumerate()
            .take_while(|(_, l)| !l.trim().is_empty())
            .fold(Warehouse::new(), |mut acc, (i, l)| {
                l.chars().enumerate().for_each(|(j, c)| match c {
                    '#' => {
                        acc.walls.insert((i, j));
                    }
                    'O' => {
                        acc.boxes.insert((i, j));
                    }
                    '@' => acc.robot = (i, j),
                    _ => (),
                });
                acc
            });

        warehouse
    }
}

mod part_2 {
    use std::collections::HashSet;

    use super::{Direction, Position};

    #[derive(Clone)]
    pub struct Warehouse {
        robot: Position,
        walls: HashSet<Position>,
        boxes: HashSet<Position>,
        right_boxes: HashSet<Position>,
    }

    impl Warehouse {
        pub fn new() -> Self {
            Self {
                robot: (0, 0),
                walls: HashSet::new(),
                boxes: HashSet::new(),
                right_boxes: HashSet::new(),
            }
        }
        pub fn move_robot(&mut self, dir: &Direction) {
            let new_pos = dir.move_position_towards(self.robot);

            if self.walls.contains(&new_pos) {
                // Against a wall, do nothing
                return;
            }

            if self.boxes.contains(&new_pos) {
                // Against a box, try propagating
                if !self.move_box(new_pos, dir) {
                    return;
                }
            } else if self.right_boxes.contains(&new_pos) {
                // Against a right box, try propagating but to the left
                if !self.move_box((new_pos.0, new_pos.1 - 1), dir) {
                    return;
                }
            }

            self.robot = new_pos;
        }

        fn move_box(&mut self, pos: Position, dir: &Direction) -> bool {
            assert!(
                !self.right_boxes.contains(&pos),
                "Should never be called against a right box."
            );

            match dir {
                Direction::Left | Direction::Right => {
                    if self.walls.contains(&pos) {
                        return false;
                    }
                }
                Direction::Up | Direction::Down => {
                    if self.walls.contains(&pos) || self.walls.contains(&(pos.0, pos.1 + 1)) {
                        return false;
                    }
                }
            }

            let mut pos = pos;
            match dir {
                Direction::Left | Direction::Right => {
                    if !self.boxes.contains(&pos) {
                        return true;
                    }
                }
                Direction::Up | Direction::Down => {
                    if !self.boxes.contains(&pos) {
                        if !self.boxes.contains(&(pos.0, pos.1 + 1)) {
                            return true;
                        } else {
                            pos = (pos.0, pos.1 + 1)
                        }
                    }
                }
            }

            // "We" are a left part box.
            let new_pos = dir.move_position_towards(pos);
            let mut check_pos = new_pos;

            match dir {
                Direction::Left | Direction::Right => {
                    // Ensure right part is ignored
                    if *dir == Direction::Right {
                        check_pos.1 += 1;
                    }

                    // Ensure checking part is left
                    if self.right_boxes.contains(&check_pos) {
                        check_pos = (check_pos.0, check_pos.1 - 1);
                    }

                    // Propagate move
                    let next_moved = self.move_box(check_pos, dir);

                    // Do move
                    if next_moved {
                        self.boxes.remove(&pos);
                        self.right_boxes.remove(&(pos.0, pos.1 + 1));

                        self.boxes.insert(new_pos);
                        self.right_boxes.insert((new_pos.0, new_pos.1 + 1));
                    }

                    next_moved
                }
                Direction::Up | Direction::Down => {
                    let mut check_positions = Vec::new();

                    // If the top/bottom right is a wall, stop (left is checked earlier)
                    if self.walls.contains(&(check_pos.0, check_pos.1 + 1)) {
                        return false;
                    }

                    // Ensure checking part is left
                    if self.right_boxes.contains(&check_pos) {
                        check_positions.push((check_pos.0, check_pos.1 - 1));
                    } else {
                        check_positions.push(check_pos);
                    }

                    // If top/bottom right is a box (must be left part), add it
                    if self.boxes.contains(&(check_pos.0, check_pos.1 + 1)) {
                        check_positions.push((check_pos.0, check_pos.1 + 1));
                    }

                    // Save pre-propagated state
                    let boxes = self.boxes.clone();
                    let right_boxes = self.right_boxes.clone();

                    // Check positions (will be one or two)
                    let all_moved = check_positions.iter().all(|p| self.move_box(*p, dir));

                    if all_moved {
                        self.boxes.remove(&pos);
                        self.right_boxes.remove(&(pos.0, pos.1 + 1));

                        self.boxes.insert(new_pos);
                        self.right_boxes.insert((new_pos.0, new_pos.1 + 1));
                    } else {
                        // Reverte propagated box moves
                        self.boxes = boxes;
                        self.right_boxes = right_boxes;
                    }

                    all_moved
                }
            }
        }

        pub fn get_sum(&self) -> usize {
            self.boxes.iter().map(|pos| pos.0 * 100 + pos.1).sum()
        }
    }

    pub fn parse(input_data: &str) -> Warehouse {
        let warehouse = input_data
            .lines()
            .enumerate()
            .take_while(|(_, l)| !l.trim().is_empty())
            .fold(Warehouse::new(), |mut acc, (i, l)| {
                l.chars().enumerate().for_each(|(j, c)| match c {
                    '#' => {
                        acc.walls.insert((i, j * 2));
                        acc.walls.insert((i, j * 2 + 1));
                    }
                    'O' => {
                        acc.boxes.insert((i, j * 2));
                        acc.right_boxes.insert((i, j * 2 + 1));
                    }
                    '@' => acc.robot = (i, j * 2),
                    _ => (),
                });
                acc
            });

        warehouse
    }
}

pub fn part_1(input: &Input) -> i64 {
    let mut warehouse = input.warehouse_1.clone();
    let directions = &input.directions;

    directions.iter().for_each(|d| {
        warehouse.move_robot(d);
    });

    warehouse.get_sum() as i64
}

pub fn part_2(input: &Input) -> i64 {
    let mut warehouse = input.warehouse_2.clone();
    let directions = &input.directions;

    directions.iter().for_each(|d| {
        warehouse.move_robot(d);
    });

    warehouse.get_sum() as i64
}
