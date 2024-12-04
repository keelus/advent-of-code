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

    // Check verticals
    for i in 0..line_count {
        for j in 0..line_width {
            let mut collect_check_and_count = |offset_mult: (isize, isize), condition: bool| {
                if !condition {
                    return;
                }

                let mut word = Vec::with_capacity(TARGET_WORD_LEN);

                for offset in 0..TARGET_WORD_LEN {
                    let row_offset = offset as isize * offset_mult.0;
                    let col_offset = offset as isize * offset_mult.1;

                    let row = i as isize + row_offset;
                    let col = j as isize + col_offset;

                    word.push(lines[row as usize].as_bytes()[col as usize]);
                }

                if word == TARGET_WORD.as_bytes() || word == REV_TARGET_WORD.as_bytes() {
                    count += 1;
                }
            };

            collect_check_and_count((0, 1), j <= line_width - TARGET_WORD_LEN); // Check horizontally
            if i <= line_count - TARGET_WORD_LEN {
                collect_check_and_count((1, 0), true); // Check vertically
                collect_check_and_count((1, 1), j < line_width - TARGET_WORD_LEN + 1); // Diagonally (left to right)
                collect_check_and_count((1, -1), j >= TARGET_WORD_LEN - 1); // Diagonally (right to left)
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

    let collect_and_check = |indexes: (usize, usize), offset_mult: (isize, isize)| -> bool {
        let mut word = Vec::with_capacity(TARGET_WORD_2_LEN);

        for offset in 0..TARGET_WORD_2_LEN {
            let row_offset = offset as isize * offset_mult.0;
            let col_offset = offset as isize * offset_mult.1;

            let row = indexes.0 as isize + row_offset;
            let col = indexes.1 as isize + col_offset;

            word.push(lines[row as usize].as_bytes()[col as usize]);
        }

        word == TARGET_WORD_2.as_bytes() || word == REV_TARGET_WORD_2.as_bytes()
    };

    for i in 0..(lines.len() - TARGET_WORD_2_LEN + 1) {
        for j in 0..(lines[0].len() - TARGET_WORD_2_LEN + 1) {
            if collect_and_check((i, j), (1, 1)) {
                if collect_and_check((i, j + TARGET_WORD_2_LEN - 1), (1, -1)) {
                    count += 1;
                }
            }
        }
    }

    count
}
