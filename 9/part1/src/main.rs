use std::{collections::HashSet, fs::File, io::{BufReader, BufRead}, ops::{Add, Sub}, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl Coordinate {
    fn abs(self) -> Coordinate {
        Coordinate {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl From<(i64, i64)> for Coordinate {
    fn from((x, y): (i64, i64)) -> Self {
        Coordinate { x, y }
    }
}

impl Into<(i64, i64)> for Coordinate {
    fn into(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<(i64, i64)> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        self + Coordinate::from(rhs)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}



fn do_move(direction: Direction, head: Coordinate, tail: Coordinate) -> (Coordinate, Coordinate) {
    let new_head = match direction {
        Direction::Up => head + (0, 1),
        Direction::Down => head + (0, -1),
        Direction::Left => head + (-1, 0),
        Direction::Right => head + (1, 0)
    };

    let difference = new_head - tail;

    let mut to_move = Coordinate::from((0, 0));

    if difference.abs().x == 2 || difference.abs().y == 2 {
        to_move = to_move + (difference.x.signum(), difference.y.signum());
    }

    let new_tail = tail + to_move;

    (new_head, new_tail)
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let mut visited_locations = HashSet::<Coordinate>::new();
    let mut head = Coordinate::from((0, 0));
    let mut tail = Coordinate::from((0, 0));

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let command_str: Vec<&str> = line.split_whitespace().collect();

                let command = Direction::from_str(command_str[0]).unwrap();
                let repeat = command_str[1].parse::<i64>().unwrap();

                for _ in 0..repeat {
                    (head, tail) = do_move(command, head, tail);

                    visited_locations.insert(tail);
                }
            },
            Err(err) => panic!("Problem reading line: {:?}", err),
        }
    }
    println!("{}", visited_locations.len());
}
