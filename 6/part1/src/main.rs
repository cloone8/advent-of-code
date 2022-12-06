use std::{fs::File, io::{BufReader, Read}, collections::{VecDeque, HashSet}};

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let mut queue: VecDeque<u8> = VecDeque::new();
    let mut num_char: usize = 0;

    for input_byte in reader.bytes() {
        let byte = match input_byte {
            Ok(byte) => byte,
            Err(error) => panic!("Problem reading the input file: {:?}", error),
        };

        num_char += 1;
        queue.push_back(byte);

        if queue.len() < 4 {
            continue;
        }

        let set: HashSet<u8> = queue.iter().take(4).cloned().collect();

        if set.len() == 4 {
            println!("{}", num_char);
            break;
        }

        queue.pop_front();
    }
}
