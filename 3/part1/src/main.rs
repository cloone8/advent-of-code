use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

struct Backpack {
    compartments: Vec<HashSet<u8>>,
}

fn get_prio(char: u8) -> u8 {
    if char >= 0x41 && char <= 0x5A {
        // Uppercase
        return char - 38;
    } else if char >= 0x61 && char <= 0x7A {
        // Lowercase
        return char - 96;
    } else {
        panic!("Invalid char");
    }
}

fn parse_backpacks(input: impl BufRead) -> Vec<Backpack> {
    let mut to_ret = Vec::<Backpack>::new();

    for line in input.lines() {
        match line {
            Ok(line) => {
                let compartments = line.split_at(line.len() / 2);

                let new_backpack = Backpack {
                    compartments: vec![
                        compartments.0.bytes().map(|b| get_prio(b)).collect(),
                        compartments.1.bytes().map(|b| get_prio(b)).collect(),
                    ],
                };

                to_ret.push(new_backpack);
            }
            Err(error) => panic!("Could not read line: {:?}", error),
        }
    }

    to_ret
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let backpacks = parse_backpacks(reader);

    let mut total_sum: u64 = 0;
    for backpack in backpacks.iter() {
        let compartment_0 = &backpack.compartments[0];
        let compartment_1 = &backpack.compartments[1];

        compartment_0
            .intersection(compartment_1)
            .for_each(|item| total_sum += *item as u64);
    }

    println!("{}", total_sum);
}
