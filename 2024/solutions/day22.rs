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

pub fn part_2(_secret_numbers: &[u64]) -> i64 {
    0
}
