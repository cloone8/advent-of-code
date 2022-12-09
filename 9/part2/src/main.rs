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

impl From<Direction> for Coordinate {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Coordinate::from((0, 1)),
            Direction::Down => Coordinate::from((0, -1)),
            Direction::Left => Coordinate::from((-1, 0)),
            Direction::Right => Coordinate::from((1, 0)),
        }
    }
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

fn do_move(head: Coordinate, tail: Coordinate) -> Coordinate {
    let difference = head - tail;

    if difference.abs().x == 2 || difference.abs().y == 2 {
        tail + (difference.x.signum(), difference.y.signum())
    } else {
        tail
    }
}

fn move_rope(direction: Direction, rope: &mut Vec<Coordinate>) {
    rope[0] = rope[0] + Coordinate::from(direction);

    for i in 1..rope.len() {
        rope[i] = do_move(rope[i - 1], rope[i]);
    }
}

fn main() {
    let input_file = match File::open("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the input file: {:?}", error),
    };

    let reader = BufReader::new(input_file);

    let mut visited_locations = HashSet::<Coordinate>::new();
    let mut rope: Vec<Coordinate> = vec![Coordinate::from((0, 0)); 10];

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let command_str: Vec<&str> = line.split_whitespace().collect();

                let command = Direction::from_str(command_str[0]).unwrap();
                let repeat = command_str[1].parse::<i64>().unwrap();

                for _ in 0..repeat {
                    move_rope(command, &mut rope);

                    visited_locations.insert(rope.last().unwrap().clone());
                }
            },
            Err(err) => panic!("Problem reading line: {:?}", err),
        }
    }
    println!("{}", visited_locations.len());
}
