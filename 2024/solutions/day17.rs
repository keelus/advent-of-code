use std::fmt::Debug;

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;

#[derive(Clone)]
pub struct CpuState {
    registers: [u64; 3],
    program: Vec<u8>,
    program_counter: usize,

    halted: bool,
    output: Vec<u8>,
}

impl Debug for CpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "=== CPU STATE ===\n")?;
        write!(f, "= Register A: {}\n", self.registers[0])?;
        write!(f, "= Register B: {}\n", self.registers[1])?;
        write!(f, "= Register C: {}\n", self.registers[2])?;
        write!(f, "= Program:  {:?}\n", self.program)?;
        write!(f, "= PC:  {}\n", self.program_counter)?;
        write!(f, "= Halted:  {}\n", self.halted)
    }
}

impl CpuState {
    pub fn new() -> Self {
        Self {
            registers: [0; 3],
            program: vec![],
            program_counter: 0,
            halted: false,
            output: vec![],
        }
    }

    pub fn tick(&mut self) {
        if self.halted {
            return;
        }

        let opcode = self.program[self.program_counter];
        self.program_counter += 1;
        let operand = self.program[self.program_counter];

        let literal_operand = || operand as u64;
        let combo_operand = || match operand {
            0..=3 => self.program[self.program_counter] as u64,
            4 => self.registers[REG_A],
            5 => self.registers[REG_B],
            6 => self.registers[REG_C],
            _ => unreachable!("Invalid combo operand \"{}\".", operand),
        };

        let mut jumped = false;
        match opcode {
            0 => {
                // adv (division)
                let res = self.registers[REG_A] as f64 / 2u64.pow(combo_operand() as u32) as f64;
                self.registers[REG_A] = res as u64;
            }
            1 => {
                // bxl (bitwise XOR)
                let res = self.registers[REG_B] ^ literal_operand();
                self.registers[REG_B] = res;
            }
            2 => {
                // bst (modulo 8)
                let res = combo_operand() % 8;
                self.registers[REG_B] = res;
            }
            3 => {
                // jnz (bne a 0)
                if self.registers[REG_A] != 0 {
                    self.program_counter = literal_operand() as usize;
                    jumped = true;
                }
            }
            4 => {
                // bxc (bitwise XOR)
                let res = self.registers[REG_B] ^ self.registers[REG_C];
                self.registers[REG_B] = res;
            }
            5 => {
                // out
                let res = combo_operand() % 8;
                self.output.push(res as u8);
            }
            6 => {
                // bdv (division)
                let res = self.registers[REG_A] as f64 / 2u64.pow(combo_operand() as u32) as f64;
                self.registers[REG_B] = res as u64;
            }
            7 => {
                // cdv (division)
                let res = self.registers[REG_A] as f64 / 2u64.pow(combo_operand() as u32) as f64;
                self.registers[REG_C] = res as u64;
            }
            _ => (),
        }

        if !jumped {
            self.program_counter += 1;
        }

        if self.program_counter >= self.program.len() {
            self.halted = true;
        }
    }

    pub fn print_output(&self) {
        println!(
            "{}",
            self.output
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    }
}

pub fn parse(input_data: String) -> CpuState {
    input_data.lines().fold(CpuState::new(), |mut cpu, l| {
        match l.split_whitespace().collect::<Vec<_>>()[..] {
            ["Register", reg, val] => {
                let val = val.parse().unwrap();
                match reg {
                    "A:" => cpu.registers[0] = val,
                    "B:" => cpu.registers[1] = val,
                    "C:" => cpu.registers[2] = val,
                    _ => (),
                }
            }
            ["Program:", v] => {
                cpu.program = v.split(",").map(|i| i.parse().unwrap()).collect();
            }
            _ => (),
        };
        cpu
    })
}

pub fn part_1(cpu_state: &CpuState) -> i64 {
    let mut cpu = cpu_state.clone();
    println!("{:?}", cpu);

    while !cpu.halted {
        cpu.tick();
    }

    print!("Part 1: ");
    cpu.print_output();

    0
}

pub fn part_2(cpu_state: &CpuState) -> i64 {
    0
}
