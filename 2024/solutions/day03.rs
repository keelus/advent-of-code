pub enum Token {
    Multiplication(i64, i64),
    Do,
    Dont,
}

pub fn parse(input_data: String) -> Vec<Token> {
    let len = input_data.len();
    let mem = input_data.as_bytes();

    let mut i = 0;
    let mut tokens = vec![];

    while i < len {
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
                            tokens.push(Token::Multiplication(num_1, num_2))
                        }
                    }
                }
            }
        } else if i + 6 < len && &mem[i..=i + 6] == b"don't()" {
            tokens.push(Token::Dont);
        } else if i + 1 < len && &mem[i..=i + 1] == b"do" {
            tokens.push(Token::Do);
        }

        i += 1
    }

    tokens
}

pub fn part_1(tokens: &[Token]) -> i64 {
    tokens
        .iter()
        .filter_map(|token| match token {
            Token::Multiplication(num_1, num_2) => Some(num_1 * num_2),
            _ => None,
        })
        .sum()
}

pub fn part_2(tokens: &[Token]) -> i64 {
    let mut doing = true;
    tokens
        .iter()
        .filter_map(|token| {
            match token {
                Token::Multiplication(num_1, num_2) if doing => return Some(num_1 * num_2),
                Token::Do => doing = true,
                Token::Dont => doing = false,
                _ => (),
            }
            None
        })
        .sum()
}
