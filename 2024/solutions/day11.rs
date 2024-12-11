use std::collections::HashMap;

type Rock = u64;

pub fn parse<'p>(input_data: String) -> Vec<Rock> {
    input_data
        .split_whitespace()
        .map(|n| n.parse().expect("Invalid number to parse \"{n}\""))
        .collect()
}

fn blink_rock(rock: Rock, i: usize, cache: &mut HashMap<(Rock, usize), usize>) -> usize {
    let len = ((rock as f32).log10() as usize) + 1;

    if i == 1 {
        return if len % 2 == 0 { 2 } else { 1 };
    }

    if let Some(&v) = cache.get(&(rock, i)) {
        return v;
    }

    let v = {
        if rock == 0 {
            blink_rock(1, i - 1, cache)
        } else if len % 2 == 0 {
            let half_len = len / 2;
            let nn = 10usize.pow(half_len as u32);

            let left_half = rock / nn as u64;
            let right_half = rock % nn as u64;

            blink_rock(left_half, i - 1, cache) + blink_rock(right_half, i - 1, cache)
        } else {
            blink_rock(rock * 2024, i - 1, cache)
        }
    };

    cache.insert((rock, i), v);
    v
}

pub fn part_1(input: &[Rock]) -> i64 {
    input
        .iter()
        .map(|&n| blink_rock(n, 25, &mut HashMap::new()))
        .sum::<usize>() as i64
}

pub fn part_2(input: &[Rock]) -> i64 {
    input
        .iter()
        .map(|&n| blink_rock(n, 75, &mut HashMap::new()))
        .sum::<usize>() as i64
}
