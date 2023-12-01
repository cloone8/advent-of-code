use core::{panic};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Range = (u8, u8);

fn elf_to_range(elf_str: &str) -> Range {
    let elf: Vec<&str> = elf_str.split('-').collect();

    let start = elf[0].parse::<u8>().unwrap();
    let end = elf[1].parse::<u8>().unwrap();

    (start, end)
}

fn range_contains(range: &Range, value: u8) -> bool {
    value >= range.0 && value <= range.1
}

fn parse_elfs(input: impl BufRead) -> u64 {
    let mut num_overlaps: u64 = 0;

    for line in input.lines() {
        match line {
            Ok(line) => {
                let elfs: Vec<&str> = line.split(',').collect();

                if elfs.len() != 2 {
                    panic!("Invalid input");
                }
                let elf0_range: Range = elf_to_range(elfs[0]);
                let elf1_range: Range = elf_to_range(elfs[1]);

                if range_contains(&elf1_range, elf0_range.0) || range_contains(&elf1_range, elf0_range.1) {
                    // Elf 0 partially in elf 1
                    num_overlaps += 1;
                } else if range_contains(&elf0_range, elf1_range.0) || range_contains(&elf0_range, elf1_range.1) {
                    // Elf 1 partially in elf 0
                    num_overlaps += 1;
                }
            }
            Err(error) => panic!("Could not read line: {:?}", error),
        }
    }

    num_overlaps
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let overlaps = parse_elfs(reader);

    println!("{}", overlaps);
}
