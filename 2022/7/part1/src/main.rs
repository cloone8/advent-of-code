use std::{fs::File, io::{BufReader, BufRead, Result}};
use scanf::sscanf;

type Dir = (String, usize);

fn get_dir_size<I>(input: &mut I, dir_name: String, dirs_to_count: &mut Vec<Dir>) -> usize
where
    I: Iterator<Item = Result<String>>
    {
        let mut dir_size: usize = 0;

        while let Some(line_result) = input.next() {
            match line_result {
                Ok(line) => {
                    if line.starts_with("$") {
                        // Command
                        if line.starts_with("$ cd") {
                            // Change directory scope here. We can ignore listings
                            // as they'll be picked up automatically later

                            let mut change_to: String = String::new();

                            match sscanf!(&line, "$ cd {string}", change_to) {
                                Ok(_) => {
                                    if change_to == ".." {
                                        break; // We're done in this directory
                                    } else {
                                        // Change to the new directory and add its final
                                        // size to the current directory's size
                                        dir_size += get_dir_size(input, change_to, dirs_to_count);
                                    }
                                },
                                Err(err) => panic!("Error parsing cd command: {:?}", err),
                            }
                        }
                    } else {
                        // Listing

                        // Try to parse it as a file, as directories will be parsed recursively
                        // so their listing is irrelevant
                        let mut file_size: u64 = 0;
                        let mut file_name: String = String::new();

                        if sscanf!(&line, "{u64} {string}", file_size, file_name).is_ok() {
                            dir_size += usize::try_from(file_size).unwrap();
                        }
                        // dirs are irrelevant
                    }
                },
                Err(err) => panic!("Error reading line: {:?}", err),
            }
        }

    // Input for this directory (or total input) terminated. Return the final determined size
    if dir_size <= 100000 {
        // If below the threshold size, add the directory to the countable dir list
        dirs_to_count.push((dir_name, dir_size));
    }
    dir_size
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);
    let lines = reader.lines();
    let mut dirs_to_count: Vec<Dir> = Vec::new();

    let total_size = get_dir_size(&mut lines.skip(1), "/".to_owned(), &mut dirs_to_count);

    println!("Total size: {}\n", total_size);
    let mut sum_dirs_to_count: usize = 0;
    for dir in dirs_to_count {
        sum_dirs_to_count += dir.1;
        println!("{}: {}", dir.0, dir.1);
    }

    println!("\ncounted dir size: {}", sum_dirs_to_count);
}
