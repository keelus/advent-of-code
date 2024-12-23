use std::collections::HashMap;

pub fn parse(input_data: String) -> Vec<u64> {
    input_data.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn mix(value: u64, secret_number: u64) -> u64 {
    value ^ secret_number
}

pub fn prune(secret_number: u64) -> u64 {
    secret_number % 16777216
}

pub fn next_secret_number(secret_number: u64) -> u64 {
    let secret_number = prune(mix(secret_number * 64, secret_number));
    let secret_number = prune(mix(secret_number / 32, secret_number));
    let secret_number = prune(mix(secret_number * 2048, secret_number));
    secret_number
}

pub fn part_1(secret_numbers: &[u64]) -> i64 {
    secret_numbers
        .iter()
        .map(|&secret_number| {
            let mut secret_number = secret_number;

            (0..2000).into_iter().for_each(|_i| {
                secret_number = next_secret_number(secret_number);
            });

            secret_number
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

pub fn part_2(secret_numbers: &[u64]) -> i64 {
    let w = 4;
    let n = 2000 + w;

    let mut consecutive_changes: HashMap<[i8; 4], Vec<u64>> = HashMap::new();
    secret_numbers.iter().for_each(|&starting_secret_number| {
        let mut secret_number = starting_secret_number;
        let secret_numbers = (0..n)
            .into_iter()
            .map(|_i| {
                secret_number = next_secret_number(secret_number);
                secret_number
            })
            .collect::<Vec<u64>>();

        let (intervals, _) = (0..(n - w)).into_iter().fold(
            (Vec::new(), (starting_secret_number % 10) as i8),
            |(mut intervals, prev_first_digit), i| {
                let window = secret_numbers.get(i..(i + w)).unwrap();
                let digits: Vec<i8> = window
                    .iter()
                    .map(|secret_number| (secret_number % 10) as i8)
                    .collect();

                let d_0 = digits[0] - prev_first_digit;
                let d_1 = digits[1] - digits[0];
                let d_2 = digits[2] - digits[1];
                let d_3 = digits[3] - digits[2];

                let price_at = digits[3] as u64;

                intervals.push(([d_0, d_1, d_2, d_3], price_at));

                (intervals, digits[0])
            },
        );

        let intervals = intervals
            .into_iter()
            .fold(HashMap::new(), |mut acc, (key, val)| {
                if !acc.contains_key(&key) {
                    acc.insert(key, val);
                }

                acc
            });

        intervals.iter().for_each(|(&key, &val)| {
            consecutive_changes
                .entry(key)
                .and_modify(|v| v.push(val))
                .or_insert(vec![val]);
        });
    });

    consecutive_changes
        .into_iter()
        .max_by_key(|(_, val)| val.iter().sum::<u64>())
        .unwrap()
        .1
        .iter()
        .sum::<u64>() as i64
}
