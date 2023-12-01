use std::{collections::VecDeque, fs::File, io::{BufReader, BufRead}};
use scanf::sscanf;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    test_action: (usize, usize),
    total_inspections: u64
}

#[derive(Debug, Clone)]
enum MonkeyOperation {
    Add(u64),
    Mul(u64),
    Square()
}

impl MonkeyOperation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            MonkeyOperation::Add(operand) => old + operand,
            MonkeyOperation::Mul(operand) => old * operand,
            MonkeyOperation::Square() => old * old
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyTest {
    Div(u64)
}

impl MonkeyTest {
    fn apply(&self, item: u64) -> bool {
        match self {
            MonkeyTest::Div(divisor) => item % divisor == 0
        }
    }
}

fn parse_monkey(monkey_string: &String) -> Monkey {
    let lines = monkey_string.lines().collect::<Vec<&str>>();

    // Items
    let items = lines[1].split_whitespace()
        .skip(2)
        .map(|i| i.replace(',', ""))
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>();

    // Operation
    let mut operation_str = String::new();
    let mut operand_str = String::new();

    sscanf!(lines[2], "  Operation: new = old {string} {string}", operation_str, operand_str).unwrap();

    let operation: MonkeyOperation;

    match operand_str.as_str() {
        "old" => operation = MonkeyOperation::Square(),
        _ => {
            let operand = operand_str.parse::<u64>().unwrap();

            match operation_str.as_str() {
                "+" => operation = MonkeyOperation::Add(operand),
                "*" => operation = MonkeyOperation::Mul(operand),
                _ => panic!("Unknown operation!"),
            }
        }
    }

    // Test
    let mut divisor: u64 = 0;
    sscanf!(lines[3], "  Test: divisible by {u64}", divisor).unwrap();

    let test = MonkeyTest::Div(divisor);

    // Test action
    let mut action_true: u64 = 0;
    sscanf!(lines[4], "    If true: throw to monkey {u64}", action_true).unwrap();

    let mut action_false: u64 = 0;
    sscanf!(lines[5], "    If false: throw to monkey {u64}", action_false).unwrap();

    let test_action = (action_true as usize, action_false as usize);

    // Done!
    Monkey {
        items,
        operation,
        test,
        test_action,
        total_inspections: 0
    }
}

fn parse_monkeys(file: File) -> Vec<Monkey> {
    let mut reader = BufReader::new(file);

    let mut monkey_strings: Vec<String> = Vec::new();
    loop {
        let mut monkey_buf = String::new();

        for _ in 0..7 {
            match reader.read_line(&mut monkey_buf) {
                Ok(_) => (),
                Err(err) => panic!("Could not read line! {}", err),
            }
        }

        if monkey_buf.len() == 0 {
            break;
        }

        monkey_strings.push(monkey_buf);
    }

    monkey_strings.iter()
        .map(|m| parse_monkey(m))
        .collect()
}

fn do_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let cur_monkey = monkeys[i].clone();

        for item in cur_monkey.items {
            let new_item = cur_monkey.operation.apply(item) / 3;

            let (true_monkey, false_monkey) = cur_monkey.test_action;

            if cur_monkey.test.apply(new_item) {
                monkeys[true_monkey].items.push_back(new_item);
            } else {
                monkeys[false_monkey].items.push_back(new_item);
            }
        }

        // Items consumed! Move on
        monkeys[i].total_inspections += monkeys[i].items.len() as u64;
        monkeys[i].items.clear();
    }
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(err) => panic!("Could not open file! {}", err),
    };

    let mut monkeys = parse_monkeys(file);

    for _ in 0..20 {
        do_round(&mut monkeys);
    }

    let mut monkey_business: u64 = 1;

    monkeys.sort_by(|a, b| a.total_inspections.cmp(&b.total_inspections));

    for monkey in monkeys.iter().rev().take(2) {
        monkey_business *= monkey.total_inspections;
    }

    println!("{}", monkey_business);
}
