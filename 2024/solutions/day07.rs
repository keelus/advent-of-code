use std::fmt::format;

#[derive(Debug)]
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

pub fn generate_combinations(operators: &[char], len: usize) -> Vec<Vec<char>> {
    if len == 1 {
        return operators.iter().copied().map(|op| vec![op]).collect();
    }

    let generated = generate_combinations(operators, len - 1);
    let mut new_generated = Vec::with_capacity(generated.len() * operators.len());

    for gen_op in generated {
        for op in operators {
            let mut new_gen_op = gen_op.clone();
            new_gen_op.push(*op);
            new_generated.push(new_gen_op);
        }
    }

    new_generated
}

pub fn part_1(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter_map(|e| {
            let operators = ['+', '*'];
            let operator_orders = generate_combinations(&operators, e.numbers.len() - 1);

            let mut valid = false;
            for operator_order in operator_orders.into_iter() {
                let mut nums = e.numbers.iter();
                let mut ops = operator_order.into_iter();

                let mut acc = 0;
                let mut prev_op = None;
                while let Some(num) = nums.next() {
                    match prev_op {
                        Some(op) => {
                            match op {
                                '+' => acc = acc + num,
                                '*' => acc = acc * num,
                                _ => (),
                            }
                            prev_op = ops.next();

                            if prev_op.is_none() {
                                break;
                            }
                        }
                        None => {
                            // Will happen on first iteration
                            acc += num;
                            prev_op = Some(ops.next().unwrap());
                        }
                    }
                }

                if acc == e.result {
                    valid = true;
                    break;
                }
            }

            valid.then_some(e.result)
        })
        .sum()
}

pub fn part_2(equations: &[Equation]) -> i64 {
    equations
        .iter()
        .filter_map(|e| {
            let operators = ['+', '*', '|'];
            let operator_orders = generate_combinations(&operators, e.numbers.len() - 1);

            let mut valid = false;
            for operator_order in operator_orders.into_iter() {
                let mut nums = e.numbers.iter();
                let mut ops = operator_order.into_iter();

                let mut acc = 0;
                let mut prev_op = None;
                while let Some(num) = nums.next() {
                    match prev_op {
                        Some(op) => {
                            match op {
                                '+' => acc = acc + num,
                                '*' => acc = acc * num,
                                '|' => {
                                    let acc_new = format!("{}{}", acc, num);
                                    acc = acc_new.parse().unwrap();
                                }
                                _ => (),
                            }
                            prev_op = ops.next();

                            if prev_op.is_none() {
                                break;
                            }
                        }
                        None => {
                            // Will happen on first iteration
                            acc += num;
                            prev_op = Some(ops.next().unwrap());
                        }
                    }
                }

                if acc == e.result {
                    valid = true;
                    break;
                }
            }

            valid.then_some(e.result)
        })
        .sum()
}
