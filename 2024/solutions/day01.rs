use std::collections::HashMap;

type DoubleList = (Vec<i64>, Vec<i64>);

pub fn parse(input_data: String) -> DoubleList {
    let mut lists: DoubleList =
        input_data
            .lines()
            .fold((Vec::new(), Vec::new()), |mut lists, line| {
                let mut parts = line.split_whitespace();

                lists.0.push(parts.next().unwrap().parse().unwrap());
                lists.1.push(parts.next().unwrap().parse().unwrap());

                lists
            });

    lists.0.sort();
    lists.1.sort();

    return lists;
}

pub fn part_1(lists: &DoubleList) -> i64 {
    lists
        .0
        .iter()
        .zip(lists.1.iter())
        .map(|(num_1, num_2)| i64::abs(num_1 - num_2))
        .sum()
}

pub fn part_2(lists: &DoubleList) -> i64 {
    let frequencies = lists.1.iter().fold(HashMap::new(), |mut freqs, num| {
        freqs.entry(num).and_modify(|freq| *freq += 1).or_insert(1);
        freqs
    });

    let similarity_score = lists
        .0
        .iter()
        .map(|&num| num * frequencies.get(&num).unwrap_or(&0))
        .sum();

    similarity_score
}
