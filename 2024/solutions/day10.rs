use std::collections::HashSet;

type Position = (usize, usize);

pub struct Input {
    map: Vec<Vec<u8>>,
    trailheads: Vec<Position>,
}

pub fn parse<'p>(input_data: String) -> Input {
    let map = input_data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let trailheads = input_data
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '0')
                .map(|(j, _)| (i, j))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    Input { map, trailheads }
}

fn hike(
    map_dimensions: &(usize, usize),
    map: &[Vec<u8>],
    cur_pos: &Position,
) -> Vec<Option<Position>> {
    let value_cur = map[cur_pos.0][cur_pos.1];
    if value_cur == 9 {
        return vec![Some(*cur_pos)];
    }

    let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    dirs.iter()
        .filter_map(|dir| {
            let ni = cur_pos.0 as isize + dir.0;
            let nj = cur_pos.1 as isize + dir.1;

            if ni >= 0 && nj >= 0 {
                let (ni, nj) = (ni as usize, nj as usize);

                if ni < map_dimensions.0 && nj < map_dimensions.1 {
                    let value_at = map[ni][nj];

                    if value_at > 0 && value_cur == value_at - 1 {
                        return Some(hike(map_dimensions, map, &(ni, nj)));
                    }
                }
            }

            None
        })
        .flatten()
        .collect()
}

pub fn part_1(input: &Input) -> i64 {
    let map_dimensions = (input.map[0].len() as usize, input.map.len() as usize);
    input
        .trailheads
        .iter()
        .map(|th| {
            // Filter unique
            hike(&map_dimensions, &input.map, &th)
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
        })
        .flatten()
        .count() as i64
}

pub fn part_2(input: &Input) -> i64 {
    let map_dimensions = (input.map[0].len() as usize, input.map.len() as usize);
    input
        .trailheads
        .iter()
        .map(|th| hike(&map_dimensions, &input.map, &th))
        .flatten()
        .count() as i64
}
