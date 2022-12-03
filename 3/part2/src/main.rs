use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct ElfGroup {
    rucksacks: Vec<HashSet<u8>>,
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

fn parse_backpacks(input: impl BufRead) -> Vec<ElfGroup> {
    let mut to_ret = Vec::<ElfGroup>::new();

    let mut linecount = 0;
    let mut current_group = ElfGroup {
        rucksacks: Vec::<HashSet<u8>>::new(),
    };

    for line in input.lines() {
        match line {
            Ok(line) => {
                // let new_rucksack: HashSet<u8> =

                current_group.rucksacks.push(
                    line.bytes()
                        .map(|b| get_prio(b))
                        .collect()
                );

                if linecount == 2 {
                    to_ret.push(current_group.clone());

                    current_group = ElfGroup {
                        rucksacks: Vec::<HashSet<u8>>::new(),
                    };
                }

                linecount = (linecount + 1) % 3;
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

    let elfgroups = parse_backpacks(reader);

    let total_sum: u64 = elfgroups.iter().map(|elfgroup| {
        let total_set = elfgroup.rucksacks.iter()
        .fold(
            HashSet::new(),
            |left, right| left.union(right).cloned().collect()
        );

        let intersection = elfgroup.rucksacks.iter()
        .fold(
            total_set,
            |acc, x| acc.intersection(x).cloned().collect()
        );

        intersection.iter().fold(0_u64, |acc, x| acc + x.to_owned() as u64)
    }).sum();

    println!("{}", total_sum);
}
