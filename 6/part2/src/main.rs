use std::{fs::File, io::{BufReader, Read}, collections::{VecDeque, HashSet}};

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let mut queue: VecDeque<u8> = VecDeque::new();
    let mut num_char: usize = 0;
    let marker_len = 14;

    for input_byte in reader.bytes() {
        let byte = match input_byte {
            Ok(byte) => byte,
            Err(error) => panic!("Problem reading the input file: {:?}", error),
        };

        num_char += 1;
        queue.push_back(byte);

        if queue.len() < marker_len {
            continue;
        }

        let set: HashSet<u8> = queue.iter().take(marker_len).cloned().collect();

        if set.len() == marker_len {
            println!("{}", num_char);
            break;
        }

        queue.pop_front();
    }
}
