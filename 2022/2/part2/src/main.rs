use core::panic;
use std::{io::{BufRead, BufReader}, fs::File, str::FromStr, ops::Add};

#[derive(Copy, Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl From<i8> for Play {
    fn from(item: i8) -> Self {
        match item {
            1 => Play::Rock,
            2 => Play::Paper,
            3 => Play::Scissors,
            _ => panic!("Invalid play: {}", item)
        }
    }
}

impl FromStr for Play {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(())
        }
    }

    type Err = ();
}

impl Add<MatchResult> for Play {
    type Output = Play;

    fn add(self, rhs: MatchResult) -> Self::Output {
        Play::from((((self as i8) - 1) + (rhs as i8)).rem_euclid(3) + 1)
    }
}

enum MatchResult {
    Loss = -1,
    Draw = 0,
    Win = 1
}

impl FromStr for MatchResult {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchResult::Loss),
            "Y" => Ok(MatchResult::Draw),
            "Z" => Ok(MatchResult::Win),
            _ => Err(())
        }
    }

    type Err = ();
}

fn get_score_from_line(line: String) -> u8 {
    let game: Vec<&str> = line.split_whitespace().collect();

    let play = match Play::from_str(game[0]) {
        Ok(play) => play,
        Err(_) => panic!("Invalid play")
    };

    let desired_result = match MatchResult::from_str(game[1]) {
        Ok(response) => response,
        Err(_) => panic!("Invalid response")
    };

    let response: Play = play + desired_result;

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
