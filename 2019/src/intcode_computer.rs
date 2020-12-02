use std::io::{stdin, stdout};
use std::io::Write;

pub trait ReadWrite {
    fn input(&mut self) -> i64;
    fn output(&mut self, output: i64);
}

struct DefaultReadWrite {}

impl ReadWrite for DefaultReadWrite {
    fn input(&mut self) -> i64 {
        print!("Please provide input: ");
        stdout().flush().expect("some error message");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error while getting input");
        return input[0..input.len() - 1].parse().unwrap();
    }

    fn output(&mut self, output: i64) {
        println!("Output: {}", output);
    }
}

#[derive(Debug)]
enum Operation {
    Sum, Mul, Inp, Out, Halt, JumpIfTrue, JumpIfFalse, LessThen, Equals, RelativeBase
}

impl Operation {
    fn length(&self) -> usize {
        match self {
            Operation::Sum => 4,
            Operation::Mul => 4,
            Operation::Inp => 2,
            Operation::Out => 2,
            Operation::Halt => 1,
            Operation::JumpIfTrue => 3,
            Operation::JumpIfFalse => 3,
            Operation::LessThen => 4,
            Operation::Equals => 4,
            Operation::RelativeBase => 2
        }
    }

    fn from_op_code(op_code: i64) -> Operation {
        match op_code % 100 {
            1 => Operation::Sum,
            2 => Operation::Mul,
            3 => Operation::Inp,
            4 => Operation::Out,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThen,
            8 => Operation::Equals,
            9 => Operation::RelativeBase,
            99 => Operation::Halt,
            _ => panic!("Unrecognized operation code!")
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position, Immediate, Relative
}

impl ParameterMode {
    fn from_op_code(op_code: i64, pos: usize) -> ParameterMode {
        let mode = (op_code / (10i64.pow((pos as u32) + 2))) % 10;
        match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unrecognized parameter mode!")
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    a_mode: ParameterMode,
    b_mode: ParameterMode,
    c_mode: ParameterMode,
}

impl Instruction {

    fn parse_at(memory: &Vec<i64>, loc: usize) -> Instruction {
        Instruction {
            operation: Operation::from_op_code(memory[loc]),
            a_mode: ParameterMode::from_op_code(memory[loc], 2),
            b_mode: ParameterMode::from_op_code(memory[loc], 1),
            c_mode: ParameterMode::from_op_code(memory[loc], 0),
        }
    }

    fn get_parameter(&self, memory: &Vec<i64>, relative_base: &i64, loc: usize, param: usize) -> usize {
        match self.get_parameter_mode(param) {
            ParameterMode::Position => memory[loc + param] as usize,
            ParameterMode::Immediate => loc + param,
            ParameterMode::Relative => (relative_base + memory[loc + param]) as usize
        }
    }

    fn get_parameter_mode(&self, param: usize) -> &ParameterMode {
        match param {
            1 => &self.c_mode,
            2 => &self.b_mode,
            3 => &self.a_mode,
            _ => panic!("Unknown paramater")
        }
    }

    fn execute(&self, memory: &mut Vec<i64>, relative_base: &mut i64, loc: usize, readwrite: &mut impl ReadWrite) -> usize {
        match &self.operation {
            Operation::Sum => {
                let memloc = self.get_parameter(memory, relative_base, loc, 3);
                memory[memloc as usize] = memory[self.get_parameter(memory, relative_base, loc, 1)] + memory[self.get_parameter(memory, relative_base, loc, 2)];
            }
            Operation::Mul => {
                let memloc = self.get_parameter(memory, relative_base, loc, 3);
                memory[memloc as usize] = memory[self.get_parameter(memory, relative_base, loc, 1)] * memory[self.get_parameter(memory, relative_base, loc, 2)];
            }
            Operation::Inp => {
                let memloc = self.get_parameter(memory, relative_base, loc, 1);
                let inp = readwrite.input();
                memory[memloc as usize] = inp
            }
            Operation::Out => {
                readwrite.output(memory[self.get_parameter(memory, relative_base, loc, 1)])
            }
            Operation::Halt => {
                return usize::max_value()
            }
            Operation::JumpIfFalse => {
                if memory[self.get_parameter(memory, relative_base, loc, 1)] == 0 {
                    return memory[self.get_parameter(memory, relative_base, loc, 2)] as usize;
                }
            }
            Operation::JumpIfTrue => {
                if memory[self.get_parameter(memory, relative_base, loc, 1)] != 0 {
                    return memory[self.get_parameter(memory, relative_base, loc, 2)] as usize;
                }
            }
            Operation::LessThen => {
                let memloc = self.get_parameter(memory, relative_base, loc, 3) as usize;
                if memory[self.get_parameter(memory, relative_base, loc, 1)] < memory[self.get_parameter(memory, relative_base, loc, 2)] {
                    memory[memloc] = 1
                } else {
                    memory[memloc] = 0
                }
            }
            Operation::Equals => {
                let memloc = self.get_parameter(memory, relative_base, loc, 3) as usize;
                if memory[self.get_parameter(memory, relative_base, loc, 1)] == memory[self.get_parameter(memory, relative_base, loc, 2)] {
                    memory[memloc] = 1
                } else {
                    memory[memloc] = 0
                }
            }
            Operation::RelativeBase => {
                *relative_base += memory[self.get_parameter(memory, relative_base, loc, 1)];
            }
        }
        return loc + self.operation.length()
    }
}

pub fn simulate_computer(memory: Vec<i64>) -> i64 {
   return simulate_computer_with_read_write(memory, &mut DefaultReadWrite {});
}

pub fn simulate_computer_with_read_write(mut memory: Vec<i64>, readwrite: &mut impl ReadWrite) -> i64 {

    memory.resize_with(4096, || 0);
    let mut instruction_pointer: usize = 0;
    let mut relative_base: i64 = 0;
    
    loop {
        let instruction = Instruction::parse_at(&mut memory, instruction_pointer);
        instruction_pointer = instruction.execute(&mut memory, &mut relative_base, instruction_pointer, readwrite);
        if instruction_pointer == usize::max_value() {
            break;
        }
    }

    return memory[0]
}

#[allow(dead_code)]
pub fn print_memory(memory: &Vec<usize>) {
    let memory: Vec<String> = memory.iter().map(|x| x.to_string()).collect();
    println!("{}", memory.join(","));
}