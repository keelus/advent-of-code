pub fn parse(input_data: String) -> String {
    /* Done in each part, as they differ */
    input_data
}

pub fn part_1(memory: &str) -> i64 {
    let len = memory.len();
    let mem = memory.as_bytes();

    let mut i = 0;
    let mut nums = vec![];
    loop {
        if i >= len - 1 {
            break;
        }

        if i + 3 < len && &mem[i..=i + 3] == b"mul(" {
            i += 4;

            let consume_digits = |begin_index: usize, target_end: u8| -> (usize, Option<Vec<u8>>) {
                if begin_index >= len {
                    return (begin_index, None);
                }

                let mut index = begin_index;

                let num_str: Vec<u8> = mem[index..]
                    .iter()
                    .take_while(|&c| c.is_ascii_digit())
                    .map(|c| {
                        index += 1;
                        *c
                    })
                    .collect();

                (
                    index,
                    if index < len && mem[index] == target_end {
                        Some(num_str)
                    } else {
                        None
                    },
                )
            };

            if let (new_index, Some(num_1_str)) = consume_digits(i, b',') {
                i = new_index + 1;
                if let (new_index, Some(num_2_str)) = consume_digits(i, b')') {
                    i = new_index;

                    if let Ok(num_1) = String::from_utf8(num_1_str).unwrap().parse::<i64>() {
                        if let Ok(num_2) = String::from_utf8(num_2_str).unwrap().parse::<i64>() {
                            nums.push(num_1 * num_2);
                        }
                    }
                }
            }
        }

        i += 1
    }
    nums.into_iter().sum()
}

// Same as part 1, but with "Don't()" and "Do()" detection.
pub fn part_2(memory: &str) -> i64 {
    let len = memory.len();
    let mem = memory.as_bytes();

    let mut i = 0;
    let mut nums = vec![];
    let mut doing = true;
    loop {
        if i >= len - 1 {
            break;
        }

        if i + 3 < len && &mem[i..=i + 3] == b"mul(" {
            i += 4;

            let consume_digits = |begin_index: usize, target_end: u8| -> (usize, Option<Vec<u8>>) {
                if begin_index >= len {
                    return (begin_index, None);
                }

                let mut index = begin_index;

                let num_str: Vec<u8> = mem[index..]
                    .iter()
                    .take_while(|&c| c.is_ascii_digit())
                    .map(|c| {
                        index += 1;
                        *c
                    })
                    .collect();

                (
                    index,
                    if index < len && mem[index] == target_end {
                        Some(num_str)
                    } else {
                        None
                    },
                )
            };

            if let (new_index, Some(num_1_str)) = consume_digits(i, b',') {
                i = new_index + 1;
                if let (new_index, Some(num_2_str)) = consume_digits(i, b')') {
                    i = new_index;

                    if !doing {
                        continue;
                    }

                    if let Ok(num_1) = String::from_utf8(num_1_str).unwrap().parse::<i64>() {
                        if let Ok(num_2) = String::from_utf8(num_2_str).unwrap().parse::<i64>() {
                            nums.push(num_1 * num_2);
                        }
                    }
                }
            }
        } else if i + 6 < len && &mem[i..=i + 6] == b"don't()" {
            doing = false;
        } else if i + 1 < len && &mem[i..=i + 1] == b"do" {
            doing = true;
        }

        i += 1
    }

    nums.into_iter().sum()
}
