use crate::puzzle::Puzzle;

pub(crate) struct Day17;
impl Puzzle for Day17 {
    type Part1 = String;
    type Part2 = u64;

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (initial_registers, program) = inp.split_once("\n\n").unwrap();

        let program: Vec<u8> = program
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let initial_registers: Vec<u64> = initial_registers
            .split('\n')
            .map(|line| line.split_once(": ").unwrap().1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let result: Vec<u8> = execute(initial_registers.try_into().unwrap(), &program);
        let part_one: String = result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut valid: Vec<u64> = vec![];

        // We initially are going to try 12 bits of information
        // Our initial B will be 5 (removing the first 5 bits from A),
        // and then 8 bits of A will be important for our mask
        let initial = 12;

        for i in 0..(1 << initial) {
            if execute([i, 0, 0], &program)[0] == program[0] {
                valid.push(i);
            }
        }

        for i in 1..program.len() {
            let mut new_valid = vec![];

            // We shift A by three bits every round, so we try to add 3 bits of information
            for v in valid {
                for j in 0..(1 << 3) {
                    let augmented_v = v + (j << (initial + (3 * (i - 1))));
                    let res = execute([augmented_v, 0, 0], &program);
                    if res.len() > i && res[i] == program[i] {
                        new_valid.push(augmented_v);
                    }
                }
            }

            valid = new_valid;
        }

        // Lowest answer
        let part_two = *valid.iter().min().unwrap();

        (part_one, part_two)
    }
}

fn get_combo(registers: &[u64; 3], value: u8) -> u64 {
    match value {
        0..=3 => value as u64,
        4..=6 => registers[(value - 4) as usize],
        _ => panic!("Unknown combo value"),
    }
}

fn execute(mut registers: [u64; 3], program: &[u8]) -> Vec<u8> {
    let mut instruction_pointer = 0;
    let mut output: Vec<u8> = Vec::new();

    while let Some(opcode) = program.get(instruction_pointer) {
        let operand = *program.get(instruction_pointer + 1).unwrap();

        match opcode {
            0 => registers[0] >>= get_combo(&registers, operand),
            1 => registers[1] ^= operand as u64, // bxl: bitwise XOR B/literal
            2 => registers[1] = get_combo(&registers, operand).rem_euclid(8), // bst: mod 8
            3 => {
                // jnz: jump if not zero
                if registers[0] != 0 {
                    instruction_pointer = operand as usize;
                    continue; // We skip increasing the instruction_pointer;
                }
            }
            4 => registers[1] ^= registers[2], // bxc: bitwise XOR B/C
            5 => output.push(get_combo(&registers, operand).rem_euclid(8) as u8), // out: outputs combo
            6 => registers[1] = registers[0] >> get_combo(&registers, operand), // bdv: division store to B
            7 => registers[2] = registers[0] >> get_combo(&registers, operand), // cdv: division store to C
            _ => panic!("unknown opcode"),
        }

        instruction_pointer += 2;
    }

    output
}
