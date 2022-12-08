use std::{fs::File, io::{BufReader, BufRead, Seek}};

fn get_forest_dims(input: &mut (impl BufRead + Seek)) -> (usize, usize) {
    let mut height = 0;
    let mut width = 0;

    for line_res in input.lines() {
        match line_res {
            Ok(line) => {
                height += 1;
                width = line.len();
            },
            Err(error) => panic!("Problem reading the input file: {:?}", error),
        }
    }

    input.rewind().unwrap();

    (width, height)
}

fn init_forest(input: &mut (impl BufRead + Seek), width: usize, height: usize) -> Vec<Vec<u8>> {
    let mut forest = vec![vec![0; width]; height];

    let mut row: usize = 0;

    for line_res in input.lines() {
        match line_res {
            Ok(line) => {
                let mut col: usize = 0;

                for char in line.chars() {
                    forest[row][col] = match char.to_digit(10) {
                        Some(height_32) => u8::try_from(height_32).unwrap(),
                        None => panic!("Could not read input char {}", char),
                    };

                    col += 1;
                }

                row += 1;
            },
            Err(error) => panic!("Problem reading the input file: {:?}", error),
        }
    }

    input.rewind().unwrap();

    forest
}

fn get_ss_left(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> u64 {
    let tree_height = forest[row][col];

    let mut ss_left: u64 = 0;

    for i in (0..col).rev() {
        ss_left += 1;

        let other_tree = forest[row][i];

        if other_tree >= tree_height {
            break;
        }
    }

    ss_left
}

fn get_ss_right(forest: &Vec<Vec<u8>>, width: usize, row: usize, col: usize) -> u64{
    let tree_height = forest[row][col];

    let mut ss_right: u64 = 0;

    for i in (col + 1)..width {
        ss_right += 1;

        let other_tree = forest[row][i];

        if other_tree >= tree_height {
            break;
        }
    }

    ss_right
}

fn get_ss_bottom(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> u64{
    let tree_height = forest[row][col];

    let mut ss_bot: u64 = 0;

    for i in (0..row).rev() {
        ss_bot += 1;

        let other_tree = forest[i][col];

        if other_tree >= tree_height {
            break;
        }
    }

    ss_bot
}

fn get_ss_top(forest: &Vec<Vec<u8>>, height: usize, row: usize, col: usize) -> u64 {
    let tree_height = forest[row][col];

    let mut ss_top: u64 = 0;

    for i in (row + 1)..height {
        ss_top += 1;

        let other_tree = forest[i][col];

        if other_tree >= tree_height {
            break;
        }
    }

    ss_top
}

fn get_scenic_score(forest: &Vec<Vec<u8>>, width: usize, height: usize, row: usize, col: usize) -> u64 {
    get_ss_left(forest, row, col) *
    get_ss_right(forest, width, row, col) *
    get_ss_bottom(forest, row, col) *
    get_ss_top(forest, height, row, col)
}

fn get_highest_scenic_score(forest: Vec<Vec<u8>>, width: usize, height: usize) -> u64 {
    let mut highest_score = 0;

    for row in 0..height {
        for col in 0..width {
            let score = get_scenic_score(&forest, width, height, row, col);

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let mut reader = BufReader::new(input_file);

    let (width, height) = get_forest_dims(&mut reader);
    let forest = init_forest(&mut reader, width, height);

    let highest_scenic_score = get_highest_scenic_score(forest, width, height);

    println!("Highest scenic score: {}", highest_scenic_score);
}
