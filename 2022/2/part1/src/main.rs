use core::panic;
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
        _ => panic!("Invalid play: {}", game[0].as_bytes()[0])
    };

    let response = match game[1].as_bytes()[0] {
        0x58 => Response::Rock,
        0x59 => Response::Paper,
        0x5A => Response::Scissors,
        _ => panic!("Invalid response: {}", game[1].as_bytes()[0])
    };

    let play_score = play as i8;
    let response_score = response as i8;

    let match_score_i8 = ((response_score) - (play_score) + 1).rem_euclid(3) * 3;

    let match_score = match u8::try_from(match_score_i8) {
        Ok(score) => score,
        Err(_) => panic!("Invalid score: {} {} {}", play_score, response_score, match_score_i8)
    };

    match_score + u8::try_from(response_score).unwrap()
}

fn parse_score(input: impl BufRead) -> u64 {
    let mut score: u64 = 0;

    for line in input.lines() {
        match line {
            Ok(line) => {
                score += u64::from(get_score_from_line(line));
            },
            Err(error) => panic!("Could not read line: {:?}", error),
        }
    }

    score
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let final_score = parse_score(reader);

    println!("Final score: {}", final_score);
}
