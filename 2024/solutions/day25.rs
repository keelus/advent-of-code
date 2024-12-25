pub struct Input {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

pub fn parse(input_data: String) -> Input {
    let (keys, locks, _) = input_data.lines().chain(std::iter::once("")).fold(
        (Vec::new(), Vec::new(), None::<Vec<Vec<char>>>),
        |(mut keys_acc, mut locks_acc, mut cur_acc), line| {
            if line.is_empty() {
                if let Some(cur_data) = cur_acc.take() {
                    let heights = cur_data.get(1..(cur_data.len() - 1)).unwrap().iter().fold(
                        [0; 5],
                        |mut acc, line| {
                            line.iter().enumerate().for_each(|(i, &c)| {
                                if c == '#' {
                                    acc[i] += 1;
                                }
                            });

                            acc
                        },
                    );

                    if cur_data[0] == ['#'; 5] && cur_data[cur_data.len() - 1] == ['.'; 5] {
                        // Is lock
                        locks_acc.push(heights);
                    } else {
                        keys_acc.push(heights);
                    }
                }
            } else {
                if let Some(ref mut cur_acc) = cur_acc {
                    cur_acc.push(line.chars().collect());
                } else {
                    cur_acc = Some(vec![line.chars().collect()]);
                }
            }

            (keys_acc, locks_acc, cur_acc)
        },
    );

    Input { keys, locks }
}

pub fn part_1(input: &Input) -> i64 {
    input
        .locks
        .iter()
        .map(|lock| {
            input
                .keys
                .iter()
                .filter(|key| !lock.iter().enumerate().any(|(i, v)| v + key[i] >= 6))
                .count()
        })
        .sum::<usize>() as i64
}

pub fn part_2(input: &Input) -> i64 {
    0
}
