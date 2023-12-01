use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

type Elf = u64;

fn parse_elfs(input: File, elfs: &mut Vec<Elf>) {
    let reader = BufReader::new(input);

    let mut elf: Elf = 0;

    for line in reader.lines() {
        match line {
            Ok(calory_line) => {
                if !calory_line.is_empty() {
                    elf += calory_line.parse::<Elf>().unwrap();
                } else {
                    elfs.push(elf);
                    elf = 0;
                }
            },
            Err(error) => panic!("Could not read line: {:?}", error),
        }
    }
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let mut elfs = Vec::<Elf>::new();

    parse_elfs(input_file, &mut elfs);

    let max = match elfs.iter().max() {
        Some(max) => max,
        None => panic!("No max found"),
    };

    println!("Max calories:  {:?}", max);
}
