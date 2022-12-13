use std::{fs::File, io::Read, cmp::min};

#[derive(Debug, Clone)]
enum Packet {
    Int(u64),
    Packet(Vec<Packet>)
}

impl Packet {
    fn is_int(&self) -> bool {
        match self {
            Packet::Int(_) => true,
            Packet::Packet(_) => false,
        }
    }

    fn get_int(&self) -> u64 {
        match self {
            Packet::Int(i) => *i,
            Packet::Packet(_) => panic!("Packet is not an int"),
        }
    }

    fn is_packet(&self) -> bool {
        !self.is_int()
    }
}

#[derive(Debug, Clone)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

fn parse_packet(packet_str: &str) -> Packet {
    let is_packet = packet_str.starts_with("[") && packet_str.ends_with("]");

    let packet = if !is_packet {
        Packet::Int(packet_str.parse::<u64>().unwrap())
    } else {
        let trimmed_packet_str = &packet_str[1..packet_str.len() - 1];

        let mut subpacket_strs: Vec<String> = Vec::new();

        let mut iter = trimmed_packet_str.chars();
        let mut next_char = iter.next();
        let mut cur_str = String::new();

        while next_char != None {
            let c = next_char.unwrap();

            match c {
                '[' => {
                    let mut subpacket_str = String::new();
                    let mut depth = 1;
                    loop {
                        subpacket_str.push(next_char.unwrap());
                        next_char = iter.next();

                        if next_char == None {
                            panic!("Invalid packet: {}", packet_str);
                        }

                        match next_char.unwrap() {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => (),
                        }

                        if depth == 0 {
                            subpacket_str.push(next_char.unwrap());
                            break;
                        }
                    };

                    subpacket_strs.push(subpacket_str);
                    cur_str = String::new();
                },
                ',' => {
                    if cur_str.len() != 0 {
                        subpacket_strs.push(cur_str);
                        cur_str = String::new();
                    }
                },
                _ => {
                    cur_str.push(c);
                }
            }

            next_char = iter.next();
        }

        if cur_str.len() > 0 {
            subpacket_strs.push(cur_str);
        }

        let subpackets = subpacket_strs.iter()
            .map(|s| parse_packet(s))
            .collect::<Vec<Packet>>();

        let constructed_packet = Packet::Packet(subpackets);

        constructed_packet
    };

    packet
}

fn parse_pair(pair_str: &str) -> PacketPair {
    let split = pair_str.split("\n").collect::<Vec<&str>>();

    if split.len() < 2 {
        panic!("Invalid pair size: {} ({:?})", split.len(), pair_str);
    }

    PacketPair {
        left: parse_packet(split[0]),
        right: parse_packet(split[1]),
    }
}

fn parse_lists(input: File) -> Vec<PacketPair>{
    let mut reader = std::io::BufReader::new(input);

    let mut input_str: String = String::new();

    match reader.read_to_string(&mut input_str) {
        Ok(_) => (),
        Err(e) => panic!("Could not read input file: {}", e),
    }

    let pairs_str: Vec<&str> = input_str.split("\n\n").collect();

    let pairs: Vec<PacketPair> = pairs_str.iter()
        .map(|s| parse_pair(s))
        .collect();

    pairs
}

fn is_in_right_order(pair: PacketPair) -> Option<bool> {
    let left_packet = match pair.left {
        Packet::Int(_) => panic!("Left packet is not a packet"),
        Packet::Packet(p) => p,
    };

    let right_packet = match pair.right {
        Packet::Int(_) => panic!("Right packet is not a packet"),
        Packet::Packet(p) => p,
    };

    let left_side_len = left_packet.len();
    let right_side_len = right_packet.len();
    let to_compare = min(left_side_len, right_side_len);

    for i in 0..to_compare {
        let left = &left_packet[i];
        let right = &right_packet[i];

        if left.is_int() && right.is_int() {
            let li = left.get_int();
            let ri = right.get_int();

            if li < ri {
                return Some(true);
            }

            if li > ri {
                return Some(false);
            }
        } else if left.is_packet() && right.is_packet() {
            match is_in_right_order(PacketPair {
                left: left.clone(),
                right: right.clone(),
            }) {
                Some(b) => return Some(b),
                None => (),
            }
        } else {
            if left.is_int() {
                let new_left = Packet::Packet(vec![left.clone()]);

                match is_in_right_order(PacketPair {
                    left: new_left,
                    right: right.clone(),
                }) {
                    Some(b) => return Some(b),
                    None => (),
                }
            } else if right.is_int() {
                let new_right = Packet::Packet(vec![right.clone()]);

                match is_in_right_order(PacketPair {
                    left: left.clone(),
                    right: new_right.clone(),
                }) {
                    Some(b) => return Some(b),
                    None => (),
                }
            } else {
                panic!("Invalid state");
            }
        }
    }

    if left_side_len == right_side_len {
        None
    } else {
        Some(left_side_len < right_side_len)
    }
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => panic!("Couldn't open file: {}", e),
    };

    let lists = parse_lists(file);
    let mut sum: u64 = 0;

    for i in 0..lists.len() {
        let list = lists[i].clone();

        if is_in_right_order(list).unwrap() {
            sum += u64::try_from(i).unwrap() + 1;
        }
    }

    println!("Sum: {}", sum);
}
