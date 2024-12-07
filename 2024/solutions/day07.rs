pub struct Equation {
    numbers: Vec<i64>,
    result: i64,
}

pub fn parse(input_data: String) -> Vec<Equation> {
    input_data
        .lines()
        .into_iter()
        .map(|l| {
            let parts: Vec<_> = l.splitn(2, ":").collect();

            Equation {
                result: parts[0]
                    .parse()
                    .expect(&format!("Invalid result to parse: \"{}\"", parts[0])),
                numbers: parts[1]
                    .split_whitespace()
                    .map(|n| {
                        n.parse()
                            .expect(&format!("Invalid number to parse: \"{n}\""))
                    })
                    .collect(),
            }
        })
        .collect()
}

pub fn is_valid_1(equation: &Equation, acc: i64, index: usize) -> bool {
    if index == equation.numbers.len() {
        return equation.result == acc;
    }

    let num = equation.numbers[index];
    is_valid_1(equation, acc + num, index + 1) || is_valid_1(equation, acc * num, index + 1)
}

pub fn part_1(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter_map(|e| is_valid_1(e, e.numbers[0], 1).then_some(e.result))
        .sum()
}

pub fn is_valid_2(equation: &Equation, acc: i64, index: usize) -> bool {
    if index == equation.numbers.len() {
        return equation.result == acc;
    }

    let num = equation.numbers[index];

    let acc_len = (num as f32).log10() + 1.0;
    let concatenated = acc * 10i64.pow(acc_len.floor() as u32) + num;

    is_valid_2(equation, acc + num, index + 1)
        || is_valid_2(equation, if index == 0 { 0 } else { acc } * num, index + 1)
        || is_valid_2(equation, concatenated, index + 1)
}

pub fn part_2(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter_map(|e| is_valid_2(e, e.numbers[0], 1).then_some(e.result))
        .sum()
}
