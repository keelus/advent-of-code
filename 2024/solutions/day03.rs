use regex::Regex;

pub fn parse(input_data: String) -> String {
    /* Done in each part, as they differ */
    input_data
}

pub fn part_1(memory: &str) -> i64 {
    let exp = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    exp.captures_iter(memory)
        .map(|c| {
            let n_1 = c[1].parse::<i64>().unwrap();
            let n_2 = c[2].parse::<i64>().unwrap();

            n_1 * n_2
        })
        .sum()
}

pub fn part_2(memory: &str) -> i64 {
    let exp = Regex::new(r"mul\((\d+),(\d+)\)|don\'t\(\)|do\(\)").unwrap();

    let mut doing = true;
    exp.captures_iter(memory)
        .map(|c| match &c[0] {
            "don't()" => {
                doing = false;
                0
            }
            "do()" => {
                doing = true;
                0
            }
            _ => {
                if doing {
                    let n_1 = c[1].parse::<i64>().unwrap();
                    let n_2 = c[2].parse::<i64>().unwrap();

                    n_1 * n_2
                } else {
                    0
                }
            }
        })
        .sum()
}
