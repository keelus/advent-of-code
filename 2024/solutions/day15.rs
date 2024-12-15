type Position = (usize, usize);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CellType {
    Box,
    RightBox,
    Wall,
    Air,
}

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
    warehouse_1: Warehouse,
    warehouse_2: Warehouse,
    directions: Vec<Direction>,
}

pub fn parse<'p>(input_data: String) -> Input {
    let (warehouse_1, warehouse_2) = input_data
        .lines()
        .enumerate()
        .take_while(|(_, l)| !l.trim().is_empty())
        .fold(
            (Warehouse::new(), Warehouse::new()),
            |(mut acc_1, mut acc_2), (i, l)| {
                let mut row_1 = Vec::new();
                let mut row_2 = Vec::new();

                l.chars().enumerate().for_each(|(j, c)| match c {
                    '#' => {
                        row_1.push(CellType::Wall);

                        row_2.push(CellType::Wall);
                        row_2.push(CellType::Wall);
                    }
                    'O' => {
                        row_1.push(CellType::Box);

                        row_2.push(CellType::Box);
                        row_2.push(CellType::RightBox);
                    }
                    '@' => {
                        acc_1.robot = (i, j);
                        row_1.push(CellType::Air);

                        acc_2.robot = (i, j);
                        row_2.push(CellType::Air);
                        row_2.push(CellType::Air);
                    }
                    _ => {
                        row_1.push(CellType::Air);

                        row_2.push(CellType::Air);
                        row_2.push(CellType::Air);
                    }
                });

                acc_1.map.push(row_1);
                acc_2.map.push(row_2);

                (acc_1, acc_2)
            },
        );

    Input {
        warehouse_1,
        warehouse_2,
        directions: Direction::parse_vec_from_str(
            input_data.lines().skip_while(|l| !l.is_empty()).skip(1),
        ),
    }
}

#[derive(Clone)]
pub struct Warehouse {
    robot: Position,
    map: Vec<Vec<CellType>>,
}

impl Warehouse {
    pub fn new() -> Self {
        Self {
            robot: (0, 0),
            map: Vec::new(),
        }
    }

    pub fn get_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, cell)| {
                        let pos = (i, j);
                        (*cell == CellType::Box).then_some(pos.0 * 100 + pos.1)
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn is_box(&self, pos: &Position) -> bool {
        self.map[pos.0][pos.1] == CellType::Box
    }

    pub fn is_wall(&self, pos: &Position) -> bool {
        self.map[pos.0][pos.1] == CellType::Wall
    }

    //
    // Part 1 related functions
    //
    pub fn move_robot_1(&mut self, dir: &Direction) {
        let new_pos = dir.move_position_towards(self.robot);

        if self.is_wall(&new_pos) {
            // Against a wall, do nothing
            return;
        }

        if self.is_box(&new_pos) {
            // Against a box, try propagating
            if !self.move_box_1(new_pos, dir) {
                return;
            }
        }

        self.robot = new_pos;
    }

    fn move_box_1(&mut self, pos: Position, dir: &Direction) -> bool {
        if self.is_wall(&pos) {
            return false;
        }

        if !self.is_box(&pos) {
            true
        } else {
            let new_pos = dir.move_position_towards(pos);
            let next_moved = self.move_box_1(new_pos, dir);

            if next_moved {
                self.map[pos.0][pos.1] = CellType::Air;
                self.map[new_pos.0][new_pos.1] = CellType::Box;
            }

            next_moved
        }
    }

    //
    // Part 2 related functions
    //
    pub fn is_right_box(&self, pos: &Position) -> bool {
        self.map[pos.0][pos.1] == CellType::RightBox
    }

    pub fn move_robot_2(&mut self, dir: &Direction) {
        let new_pos = dir.move_position_towards(self.robot);

        if self.is_wall(&new_pos) {
            // Against a wall, do nothing
            return;
        }

        if self.is_box(&new_pos) {
            // Against a box, try propagating
            if !self.move_box_2(new_pos, dir) {
                return;
            }
        } else if self.is_right_box(&new_pos) {
            // Against a right box, try propagating but to the left
            if !self.move_box_2((new_pos.0, new_pos.1 - 1), dir) {
                return;
            }
        }

        self.robot = new_pos;
    }

    fn move_box_2(&mut self, pos: Position, dir: &Direction) -> bool {
        assert!(
            !self.is_right_box(&pos),
            "Should never be called against a right box."
        );

        match dir {
            Direction::Left | Direction::Right => {
                if self.is_wall(&pos) {
                    return false;
                }
            }
            Direction::Up | Direction::Down => {
                if self.is_wall(&pos) || self.is_wall(&(pos.0, pos.1 + 1)) {
                    return false;
                }
            }
        }

        let mut pos = pos;
        match dir {
            Direction::Left | Direction::Right => {
                if !self.is_box(&pos) {
                    return true;
                }
            }
            Direction::Up | Direction::Down => {
                if !self.is_box(&pos) {
                    if !self.is_box(&(pos.0, pos.1 + 1)) {
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
                if self.is_right_box(&check_pos) {
                    check_pos = (check_pos.0, check_pos.1 - 1);
                }

                // Propagate move
                let next_moved = self.move_box_2(check_pos, dir);

                // Do move
                if next_moved {
                    self.map[pos.0][pos.1] = CellType::Air;
                    self.map[pos.0][pos.1 + 1] = CellType::Air;

                    self.map[new_pos.0][new_pos.1] = CellType::Box;
                    self.map[new_pos.0][new_pos.1 + 1] = CellType::RightBox;
                }

                next_moved
            }
            Direction::Up | Direction::Down => {
                let mut check_positions = Vec::new();

                // If the top/bottom right is a wall, stop (left is checked earlier)
                if self.is_wall(&(check_pos.0, check_pos.1 + 1)) {
                    return false;
                }

                // Ensure checking part is left
                if self.is_right_box(&check_pos) {
                    check_positions.push((check_pos.0, check_pos.1 - 1));
                } else {
                    check_positions.push(check_pos);
                }

                // If top/bottom right is a box (must be left part), add it
                if self.is_box(&(check_pos.0, check_pos.1 + 1)) {
                    check_positions.push((check_pos.0, check_pos.1 + 1));
                }

                // Save pre-propagated state
                let map = self.map.clone();

                // Check positions (will be one or two)
                let all_moved = check_positions.iter().all(|p| self.move_box_2(*p, dir));

                if all_moved {
                    self.map[pos.0][pos.1] = CellType::Air;
                    self.map[pos.0][pos.1 + 1] = CellType::Air;

                    self.map[new_pos.0][new_pos.1] = CellType::Box;
                    self.map[new_pos.0][new_pos.1 + 1] = CellType::RightBox;
                } else {
                    // Reverte propagated box moves
                    self.map = map;
                }

                all_moved
            }
        }
    }
}

pub fn part_1(input: &Input) -> i64 {
    let mut warehouse = input.warehouse_1.clone();
    let directions = &input.directions;

    directions.iter().for_each(|d| {
        warehouse.move_robot_1(d);
    });

    warehouse.get_sum() as i64
}

pub fn part_2(input: &Input) -> i64 {
    let mut warehouse = input.warehouse_2.clone();
    let directions = &input.directions;

    directions.iter().for_each(|d| {
        warehouse.move_robot_2(d);
    });

    warehouse.get_sum() as i64
}
