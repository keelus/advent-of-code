use std::collections::{HashMap, HashSet};

type PatternSet = HashSet<String>;
type TowelCache<'a> = HashMap<&'a str, usize>;

pub struct Input {
    patterns: PatternSet,
    desired_towels: Vec<String>,
}

pub fn parse(input_data: String) -> Input {
    let patterns = input_data
        .lines()
        .next()
        .unwrap()
        .split(", ")
        .map(|l| l.to_string())
        .collect::<HashSet<_>>();

    let desired_towels = input_data
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| l.to_string())
        .collect::<Vec<_>>();

    Input {
        patterns,
        desired_towels,
    }
}

pub fn check_1(remaining: &str, patterns: &PatternSet) -> bool {
    if remaining.len() == 0 {
        return true;
    }

    for pattern in patterns.iter() {
        if remaining.len() < pattern.len() {
            continue;
        }

        if remaining.starts_with(pattern) {
            let len = pattern.len();
            let new_remaining = remaining.get(len..).unwrap();

            if check_1(new_remaining, patterns) {
                return true;
            }
        }
    }

    false
}

pub fn check_2<'a>(remaining: &'a str, patterns: &PatternSet, cache: &mut TowelCache<'a>) -> usize {
    if remaining.len() == 0 {
        return 1;
    }

    if let Some(&count) = cache.get(remaining) {
        return count;
    }

    let mut current_count = 0;
    for pattern in patterns.iter() {
        if remaining.len() < pattern.len() {
            continue;
        }

        if remaining.starts_with(pattern) {
            let len = pattern.len();
            let new_remaining = remaining.get(len..).unwrap();

            current_count += check_2(new_remaining, patterns, cache);
        }
    }

    cache.insert(remaining, current_count);
    current_count
}

pub fn part_1(input: &Input) -> i64 {
    input
        .desired_towels
        .iter()
        .filter(|towel| check_1(towel, &input.patterns))
        .count() as i64
}

pub fn part_2(input: &Input) -> i64 {
    let mut cache = TowelCache::new();

    input
        .desired_towels
        .iter()
        .map(|towel| check_2(towel, &input.patterns, &mut cache))
        .sum::<usize>() as i64
}
