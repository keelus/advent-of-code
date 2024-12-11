type Rock = u64;

pub fn parse<'p>(input_data: String) -> Vec<Rock> {
    input_data
        .split_whitespace()
        .map(|n| n.parse().expect("Invalid number to parse \"{n}\""))
        .collect()
}

fn blink(arrangement: &[Rock]) -> Vec<Rock> {
    arrangement
        .iter()
        .map(|&n| {
            let len = ((n as f32).log10() as usize) + 1;
            if n == 0 {
                vec![1]
            } else if len % 2 == 0 {
                let half_len = len / 2;
                let nn = 10usize.pow(half_len as u32);

                let left_half = n / nn as u64;
                let right_half = n % nn as u64;

                vec![left_half, right_half]
            } else {
                vec![n * 2024]
            }
        })
        .flatten()
        .collect()
}

pub fn part_1(input: &[Rock]) -> i64 {
    let mut arrangement = input.to_vec();

    for _i in 0..25 {
        arrangement = blink(&arrangement);
    }

    arrangement.len() as i64
}

pub fn part_2(input: &[Rock]) -> i64 {
    0
}
