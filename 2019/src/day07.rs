use itertools::Itertools;
use std::fs;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;

struct ReadWriteVec {
    input: VecDeque<i64>,
    output: VecDeque<i64>
}

impl super::intcode_computer::ReadWrite for ReadWriteVec {
    fn input(&mut self) -> i64 {
        self.input.pop_front().unwrap()
    }
    fn output(&mut self, output: i64) {
        self.output.push_back(output)
    }
}

struct ReadWriteMpsc {
    rx: Receiver<i64>,
    tx: Sender<i64>
}

impl super::intcode_computer::ReadWrite for ReadWriteMpsc {
    fn input(&mut self) -> i64 {
        return self.rx.recv().unwrap();
    }

    fn output(&mut self, output: i64) {
        self.tx.send(output).unwrap();
    }
}


pub fn main() {
    let file_contents = fs::read_to_string("puzzles/07.txt").unwrap();
    let base_computer: Vec<i64> = file_contents.split(",").map(|x| x.parse().unwrap()).collect();

    let sol1 = (0..5).permutations(5).map(|perm| simulate_iteration(&base_computer, &perm)).max().unwrap();
    println!("Solution to exercise 1: {}", sol1);

    let sol2 = (5..10).permutations(5).map(|perm| simulate_continues_iteration(&base_computer, &perm)).max().unwrap();
    println!("Solution to exercise 2: {}", sol2);
}

fn simulate_iteration(base_computer: &Vec<i64>, perm: &Vec<i64>) -> i64 {

    let mut last_output = 0;
    for &phase_setting in perm {
        let mut amp = create_simulation_read_write(phase_setting, last_output);
        super::intcode_computer::simulate_computer_with_read_write(base_computer.clone(), &mut amp);
        last_output = amp.output.pop_front().unwrap();
    }

    return last_output;
}

fn create_simulation_read_write(phase_setting: i64, input_signal: i64) -> ReadWriteVec {
    let mut result = VecDeque::new();
    result.push_back(phase_setting);
    result.push_back(input_signal);
    return ReadWriteVec {
        input: result,
        output: VecDeque::new()
    };
}

fn simulate_continues_iteration(base_computer: &Vec<i64>, perm: &Vec<i64>) -> i64 {
    // First we build all threads
    // Next we send them all the phase setting
    // Third we run the simulation
    // Finally get the output

    // Build all threads
    let mut receivers = VecDeque::new();
    let mut senders = vec![];

    for i in 0..perm.len() {
        let (tx, rx) = mpsc::channel::<i64>();
        receivers.push_back((i, rx));
        senders.push(tx);
    }

    let mut threads = vec![];

    while !receivers.is_empty() {
        let (pos, rx) = receivers.pop_front().unwrap();
        let memory_clone = base_computer.clone();
        let readwrite = ReadWriteMpsc {
            rx: rx,
            tx: mpsc::Sender::clone(senders.get((pos + 1) % senders.len()).unwrap())
        };

        threads.push(thread::Builder::new().name(format!("Comp {} (perm: {})", pos, perm[pos]).to_string()).spawn(move || {
            let mut readwrite = readwrite;
            super::intcode_computer::simulate_computer_with_read_write(memory_clone, &mut readwrite);

            // Last value should still be read
            if pos == 0 {
                return Some(readwrite.rx.recv().unwrap())
            } else {
                None
            }
        }).unwrap());
    }

    // Send phase setting
    for (i, phase_setting) in perm.iter().enumerate() {
        senders[i].send(*phase_setting).unwrap();
    }

    // Send 0 as starting signal
    senders[0].send(0).unwrap();

    // Join threads
    let mut result = None;
    for thread in threads {
        let thread_res = thread.join().unwrap();
        if thread_res.is_some() {
            result = thread_res;
        }
    }

    return result.unwrap();
}