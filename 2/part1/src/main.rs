use std::{io::{BufRead, BufReader}, fs::File};

enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

enum Response {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}



fn get_score_from_line(line: String) -> u8 {
    let game: Vec<&str> = line.split_whitespace().collect();

    let play = match game[0].as_bytes()[0] {
        0x41 => Play::Rock,
        0x42 => Play::Paper,
        0x43 => Play::Scissors,
        _ => panic!("Invalid play")
    };

    let response = match game[0].as_bytes()[0] {
        0x58 => Response::Rock,
        0x59 => Response::Paper,
        0x60 => Response::Scissors,
        _ => panic!("Invalid play")
    };

    let match_score = (((play as i8) - (response as i8)) % 3) * 3;
}

fn parse_score(input: impl BufRead) {
    let mut score: u64 = 0;

    for line in input.lines() {
        match line {
            Ok(line) => {
                score += u64::from(get_score_from_line(line));
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

    let reader = BufReader::new(input_file);

    parse_score(reader);

    println!("Hello, world!");
}
