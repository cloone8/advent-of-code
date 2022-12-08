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

fn tree_visible_from_left(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let tree_height = forest[row][col];

    for i in 0..col {
        let other_tree_height = forest[row][i];

        if other_tree_height >= tree_height {
            return false;
        }
    }

    true
}

fn tree_visible_from_right(forest: &Vec<Vec<u8>>, row: usize, col: usize, width: usize) -> bool {
    let tree_height = forest[row][col];

    for i in col + 1..width {
        let other_tree_height = forest[row][i];

        if other_tree_height >= tree_height {
            return false;
        }
    }

    true
}

fn tree_visible_from_bottom(forest: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let tree_height = forest[row][col];

    for i in 0..row {
        let other_tree_height = forest[i][col];

        if other_tree_height >= tree_height {
            return false;
        }
    }

    true
}

fn tree_visible_from_top(forest: &Vec<Vec<u8>>, row: usize, col: usize, height: usize) -> bool {
    let tree_height = forest[row][col];

    for i in row + 1..height {
        let other_tree_height = forest[i][col];

        if other_tree_height >= tree_height {
            return false;
        }
    }

    true
}
fn tree_visible(forest: &Vec<Vec<u8>>, row: usize, col: usize, width: usize, height: usize) -> bool {
    // Is side tree?
    if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
        return true;
    }

    return
        tree_visible_from_left(forest, row, col) ||
        tree_visible_from_right(forest, row, col, width) ||
        tree_visible_from_bottom(forest, row, col) ||
        tree_visible_from_top(forest, row, col, height);
}

fn get_visible_trees(forest: Vec<Vec<u8>>, width: usize, height: usize) -> u64 {
    let mut visible_trees: u64 = 0;

    for row in 0..height {
        for col in 0..width {
            if tree_visible(&forest, row, col, width, height) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let mut reader = BufReader::new(input_file);

    let (width, height) = get_forest_dims(&mut reader);
    let forest = init_forest(&mut reader, width, height);

    let visible_trees = get_visible_trees(forest, width, height);

    println!("Visible trees: {}", visible_trees)
}
