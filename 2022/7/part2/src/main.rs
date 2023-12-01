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
    dirs_to_count.push((dir_name, dir_size));

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

    let total_system_space: usize = 70000000;
    let required_space: usize = 30000000;

    let available_space = total_system_space - total_size;
    let to_free = required_space - available_space;

    let mut closest_found: Dir = (String::new(), usize::MAX);

    println!("Total size: {}, available size: {}, to free: {}", total_size, available_space, to_free);

    for dir in dirs_to_count {
        if dir.1 > to_free && dir.1 < closest_found.1  {
            closest_found = dir;
        }
    }

    println!("Smallest to delete: {} with size {}", closest_found.0, closest_found.1);
}
