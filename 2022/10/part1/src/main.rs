use std::{collections::VecDeque, fs::File, io::{BufReader, BufRead}};

#[derive(Debug, Clone, Copy)]
struct Registers {
    x: i64
}

#[derive(Debug)]
enum Action {
    NoOp,
    Add(i64)
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    cycles_left: u64
}

fn parse_instructions() -> VecDeque<Instruction> {
    let mut opqueue = VecDeque::new();

    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts = line.split_whitespace().collect::<Vec<&str>>();

                let opcode = parts[0];

                match opcode {
                    "noop" => {
                        opqueue.push_back(Instruction { action: Action::NoOp, cycles_left: 1 })
                    },
                    "addx" => {
                        let operand = parts[1].parse::<i64>().unwrap();
                        opqueue.push_back(Instruction { action: Action::Add(operand), cycles_left: 2 })
                    },
                    _ => panic!("Unknown opcode: {}", opcode)
                }
            },
            Err(error) => panic!("Problem reading a line: {:?}", error)
        }
    }

    opqueue
}

fn run_cpu(init: Registers, program: VecDeque<Instruction>) {
    let mut state = init;
    let mut opqueue = program;

    let mut cur_cycle: u64 = 0;
    let mut signal_strength_sum: i64 = 0;
    let mut cur_op: Option<Instruction> = None;

    while !opqueue.is_empty() {
        cur_cycle += 1;

        if cur_cycle >= 20 && ((cur_cycle - 20) % 40) == 0 {
            signal_strength_sum += i64::try_from(cur_cycle).unwrap() * state.x;
        }

        match cur_op {
            Some(_) => {},
            None => {
                cur_op = opqueue.pop_front();
            }
        }

        let to_process = cur_op.as_mut().unwrap();

        to_process.cycles_left -= 1;

        if to_process.cycles_left == 0 {
            match to_process.action {
                Action::NoOp => {},
                Action::Add(operand) => {
                    state.x += operand
                }
            }

            cur_op = None;
        }
    }

    println!("{}", signal_strength_sum);
}

fn main() {
    let init_state = Registers { x: 1 };

    let opqueue = parse_instructions();

    run_cpu(init_state, opqueue);
}
