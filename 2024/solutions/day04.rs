pub enum Token {
    Multiplication(i64, i64),
    Do,
    Dont,
}

static TARGET_WORD: &str = "XMAS";
static TARGET_WORD_LEN: usize = TARGET_WORD.len();
static REV_TARGET_WORD: &str = "SAMX";

pub fn parse(input_data: String) -> Vec<String> {
    input_data.lines().map(|l| l.to_string()).collect()
}

pub fn part_1(lines: &[String]) -> i64 {
    let mut count = 0;

    let line_count = lines.len();
    let line_width = lines[0].len();

    // Check horizontals
    lines.iter().for_each(|l| {
        let mut count_horizontals = |target_word: &str| {
            let mut i = 0;
            while i <= l.len() - target_word.len() {
                if let Some(_) = l[i..(i + target_word.len())].find(target_word) {
                    count += 1
                }

                i += 1
            }
        };

        count_horizontals(TARGET_WORD);
        count_horizontals(REV_TARGET_WORD);
    });

    // Check verticals
    for i in 0..=line_count - TARGET_WORD_LEN {
        for j in 0..line_width {
            let mut word = Vec::with_capacity(TARGET_WORD_LEN);
            for c_i in 0..TARGET_WORD_LEN {
                word.push(lines[i + c_i].as_bytes()[j]);
            }

            if word == TARGET_WORD.as_bytes() || word == REV_TARGET_WORD.as_bytes() {
                count += 1;
            }
        }
    }

    // Check diagonals (left to right)
    for i in 0..=line_count - TARGET_WORD_LEN {
        for j in 0..(line_width - TARGET_WORD_LEN + 1) {
            let mut word = Vec::with_capacity(TARGET_WORD_LEN);
            for c_ij in 0..TARGET_WORD_LEN {
                word.push(lines[i + c_ij].as_bytes()[j + c_ij]);
            }

            if word == TARGET_WORD.as_bytes() || word == REV_TARGET_WORD.as_bytes() {
                count += 1;
            }
        }
    }

    // Check diagonals (right to left)
    for i in 0..(line_count - TARGET_WORD_LEN + 1) {
        for j in (TARGET_WORD_LEN - 1)..line_width {
            let mut word = Vec::with_capacity(TARGET_WORD_LEN);
            for c_ij in 0..TARGET_WORD_LEN {
                word.push(lines[i + c_ij].as_bytes()[j - c_ij]);
            }

            if word == TARGET_WORD.as_bytes() || word == REV_TARGET_WORD.as_bytes() {
                count += 1;
            }
        }
    }

    count as i64
}

const TARGET_WORD_2: &str = "MAS";
static TARGET_WORD_2_LEN: usize = TARGET_WORD_2.len();
const REV_TARGET_WORD_2: &str = "SAM";

pub fn part_2(lines: &[String]) -> i64 {
    let mut count = 0;

    for i in 0..(lines.len() - TARGET_WORD_2_LEN + 1) {
        for j in 0..(lines[0].len() - TARGET_WORD_2_LEN + 1) {
            let mut word = Vec::with_capacity(TARGET_WORD_2_LEN);

            for c_ij in 0..TARGET_WORD_2_LEN {
                word.push(lines[i + c_ij].as_bytes()[j + c_ij]);
            }

            if word == TARGET_WORD_2.as_bytes() || word == REV_TARGET_WORD_2.as_bytes() {
                word = Vec::with_capacity(TARGET_WORD_2_LEN);

                for c_ij in 0..TARGET_WORD_2_LEN {
                    word.push(lines[i + c_ij].as_bytes()[j + TARGET_WORD_2_LEN - 1 - c_ij]);

                    if word == TARGET_WORD_2.as_bytes() || word == REV_TARGET_WORD_2.as_bytes() {
                        count += 1
                    }
                }
            }
        }
    }

    count
}
