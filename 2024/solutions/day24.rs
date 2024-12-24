use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Clone)]
pub struct Instruction {
    wire_0: String,
    wire_1: String,
    operation: Operation,
}

impl Instruction {
    pub fn evaluate(&self, wire_data: &HashMap<String, bool>) -> Option<bool> {
        if let Some(&wire_0) = wire_data.get(&self.wire_0) {
            if let Some(&wire_1) = wire_data.get(&self.wire_1) {
                return Some(match self.operation {
                    Operation::And => wire_0 && wire_1,
                    Operation::Xor => wire_0 ^ wire_1,
                    Operation::Or => wire_0 || wire_1,
                });
            }
        }

        None
    }
}

pub struct Input {
    wire_data: HashMap<String, bool>,
    output_instructions: VecDeque<(String, Instruction)>,
}

pub fn parse(input_data: String) -> Input {
    let wire_data = input_data
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let parts: Vec<&str> = l.split(": ").collect();
            (parts[0].to_owned(), parts[1] == "1")
        })
        .collect();

    let output_instructions = input_data
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (
                parts[4].to_owned(),
                Instruction {
                    wire_0: parts[0].to_owned(),
                    wire_1: parts[2].to_owned(),
                    operation: match parts[1] {
                        "AND" => Operation::And,
                        "XOR" => Operation::Xor,
                        "OR" => Operation::Or,
                        _ => unreachable!(),
                    },
                },
            )
        })
        .collect();

    Input {
        wire_data,
        output_instructions,
    }
}

pub fn part_1(input: &Input) -> i64 {
    let mut wire_data = input.wire_data.clone();
    let mut remaining_instructions = input.output_instructions.clone();

    while let Some(current_instruction) = remaining_instructions.pop_front() {
        if let Some(new_val) = current_instruction.1.evaluate(&wire_data) {
            wire_data.insert(current_instruction.0, new_val);
        } else {
            remaining_instructions.push_back(current_instruction);
        }
    }

    let mut z_wires = wire_data
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .collect::<Vec<_>>();
    z_wires.sort_by_key(|(key, _)| *key);

    z_wires.iter().enumerate().fold(0i64, |acc, (i, (_, val))| {
        let val = if **val { 1 } else { 0 };

        acc | (val << i)
    })
}

pub fn part_2(input: &Input) -> i64 {
    0
}
