use rand::Rng;
use std::collections::{HashMap, HashSet};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum GateType {
    And,
    Or,
    Xor,
}

impl GateType {
    fn calc(&self, i1: usize, i2: usize) -> usize {
        match self {
            GateType::And => i1 & i2,
            GateType::Or => i1 | i2,
            GateType::Xor => i1 ^ i2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Gate<'a> {
    i1: &'a str,
    i2: &'a str,
    gate: GateType,
    o: &'a str,
}

impl<'a> Gate<'a> {
    fn from_str(inp: &'a str) -> Self {
        let (left, right) = inp.split_once(" -> ").unwrap();
        let mut left = left.split(' ');

        Gate {
            i1: left.next().unwrap(),
            gate: {
                match left.next().unwrap() {
                    "AND" => GateType::And,
                    "OR" => GateType::Or,
                    "XOR" => GateType::Xor,
                    _ => unreachable!(),
                }
            },
            i2: left.next().unwrap(),
            o: right,
        }
    }

    fn exec(&self, wires: &mut HashMap<&'a str, usize>) {
        let inp1 = wires.get(self.i1);
        let inp2 = wires.get(self.i2);

        if let Some(&inp1) = inp1 {
            if let Some(&inp2) = inp2 {
                wires.insert(self.o, self.gate.calc(inp1, inp2));
            }
        }
    }
}

const TEST_CASES: usize = 100;

pub(crate) struct Day24;
impl Puzzle for Day24 {
    type Part1 = usize;
    type Part2 = ();

    fn solve(&self, inp: &str) -> (Self::Part1, Self::Part2) {
        let (wires, gates) = inp.split_once("\n\n").unwrap();

        let initial_wires: HashMap<&str, usize> = wires
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(x, y)| (x, y.parse::<usize>().unwrap()))
            .collect();

        // We sort the gates to be in order of wire availability (DAG order)
        // basically: executing in this order guarantees the input wires being available
        let mut enabled_wires: HashSet<&str> = initial_wires.keys().copied().collect();
        let mut unordered_gates: HashSet<Gate> = gates.lines().map(Gate::from_str).collect();

        let mut gates: Vec<Gate> = Vec::with_capacity(unordered_gates.len());

        while !unordered_gates.is_empty() {
            unordered_gates.retain(|gate| {
                if enabled_wires.contains(gate.i1) && enabled_wires.contains(gate.i2) {
                    enabled_wires.insert(gate.o);
                    gates.push(gate.clone());
                    return false;
                }

                true
            });
        }

        std::mem::drop(enabled_wires);
        std::mem::drop(unordered_gates);

        // Part one
        let mut wires = initial_wires.clone();
        execute_gates(&gates, &mut wires);
        let part_one: usize = read_value(&wires, 'z');

        // Part two
        // Part was mostly solved by hand: you receive a broken ripple adder as an input following a _very_ specific pattern
        // untangling the logic gates gives a fairly clear overview of outputs not following this pattern indicating mistakes

        // Software implementation:
        // We can mark all logic gates which we know to be correct.
        // Test X test cases, and mark all correct z-gates and dependents as correct
        // Do a single pass where two outputs are swapped giving candidates
        // (not implemented): branch on the candidates and perform a second/third/fourth swap, saving the best candidates

        let outputs = test_many_values(&gates, &initial_wires);
        let amount_correct = outputs.iter().filter(|&&x| x == TEST_CASES).count();
        let mut correct_wires: HashSet<&str> = HashSet::new();

        for gate in gates.iter().rev() {
            if let Some(name) = gate.o.strip_prefix('z') {
                // Add o to correct if outputs is 1000
                let idx = name.parse::<usize>().unwrap();
                if outputs[idx] as usize == TEST_CASES {
                    correct_wires.insert(gate.o);
                }
            }

            if correct_wires.contains(gate.o) {
                correct_wires.insert(gate.i1);
                correct_wires.insert(gate.i2);
            }
        }

        let mut gates = gates.clone();

        for i in 0..gates.len() {
            if correct_wires.contains(gates[i].o)
                && correct_wires.contains(gates[i].i1)
                && correct_wires.contains(gates[i].i2)
            {
                continue;
            }

            'inner: for j in (i + 1)..gates.len() {
                if correct_wires.contains(gates[j].o)
                    && correct_wires.contains(gates[j].i1)
                    && correct_wires.contains(gates[j].i2)
                {
                    continue 'inner;
                }

                let tmp = gates[i].o;
                gates[i].o = gates[j].o;
                gates[j].o = tmp;

                let test = test_many_values(&gates, &wires)
                    .iter()
                    .filter(|&&x| x == TEST_CASES)
                    .count();
                if test > amount_correct + 4 {
                    println!("{}:{}", gates[i].o, gates[j].o);
                }

                let tmp = gates[i].o;
                gates[i].o = gates[j].o;
                gates[j].o = tmp;
            }
        }

        (part_one, ())
    }
}

fn test_many_values(gates: &Vec<Gate>, wires: &HashMap<&str, usize>) -> [usize; 46] {
    // Generate many test cases
    let mut rng = rand::thread_rng();

    let mut correct_outputs = [0; 46];

    for _ in 0..TEST_CASES {
        let mut wires = wires.clone();

        let left = rng.gen_range((1usize << 0)..((1usize << 45) - 1));
        let right = rng.gen_range((1usize << 0)..((1usize << 45) - 1));

        write_value(&mut wires, left, 'x');
        write_value(&mut wires, right, 'y');
        execute_gates(gates, &mut wires);

        let answer = left + right;

        // Check answer
        for (&name, &value) in wires.iter() {
            if let Some(name) = name.strip_prefix('z') {
                let idx = name.parse::<usize>().unwrap();

                if value == ((answer >> idx) & 0b1) {
                    correct_outputs[idx] += 1;
                }
            }
        }
    }

    correct_outputs
}

fn execute_gates<'a>(gates: &'a Vec<Gate>, wires: &mut HashMap<&'a str, usize>) {
    for gate in gates.iter() {
        gate.exec(wires);
    }
}

fn write_value(wires: &mut HashMap<&str, usize>, value: usize, c: char) {
    for i in 0..100 {
        if let Some(wire) = wires.get_mut(&format!("{}{:02}", c, i)[..]) {
            *wire = (value >> i) & 0b1;
        }
    }
}

fn read_value(wires: &HashMap<&str, usize>, prefix: char) -> usize {
    let mut answer: usize = 0;

    for (&name, &value) in wires.iter() {
        if let Some(name) = name.strip_prefix(prefix) {
            let idx = name.parse::<usize>().unwrap();
            answer |= value << idx;
        }
    }

    answer
}
