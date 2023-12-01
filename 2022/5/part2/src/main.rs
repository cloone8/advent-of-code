use std::{fs::File, io::{BufReader, BufRead}};
use scanf::sscanf;

fn get_init_stacks(input: impl BufRead) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        stacks.push(line.chars().collect());
    }

    stacks
}

fn do_commands(stacks: &mut Vec<Vec<char>>, commands: impl BufRead) {
    for line in commands.lines() {
        match line {
            Ok(command_string) => {
                let mut amount: usize = 0;
                let mut from: usize = 0;
                let mut to: usize = 0;

                match sscanf!(&command_string, "move {} from {} to {}", amount, from, to) {
                    Ok(_) => {
                        let mut amount_to_move = amount;

                        let mut cratemover_stack: Vec<char> = Vec::new();

                        while amount_to_move > 0 {
                            let block = stacks[from - 1].pop().unwrap();
                            cratemover_stack.push(block);
                            amount_to_move -= 1;
                        }

                        amount_to_move = amount;

                        while amount_to_move > 0 {
                            let block = cratemover_stack.pop().unwrap();
                            stacks[to - 1].push(block);
                            amount_to_move -= 1;
                        }
                    }
                    Err(err) => panic!("Could not parse command string: {}", err),
                }
            },
            Err(err) => panic!("Couldn't read line!"),
        }
    }
}

fn main() {
    let input_file_stacks = match File::open("stacks.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let input_file_commands = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader_stacks = BufReader::new(input_file_stacks);
    let reader_commands = BufReader::new(input_file_commands);

    let mut stacks = get_init_stacks(reader_stacks);

    do_commands(&mut stacks, reader_commands);

    for mut stack in stacks {
        match stack.pop() {
            Some(top) => print!("{}", top),
            None => {}// Do nothing,
        }
    }
}
