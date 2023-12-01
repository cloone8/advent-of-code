use std::{fs::File, io::{BufReader, BufRead}, cmp::min};

use scanf::sscanf;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Sensor {
    location: Coord,
    closest: Coord,
}

impl Coord {
    fn distance(&self, other: &Coord) -> i64 {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();

        x + y
    }
}

impl Sensor {
    fn covers(&self, other: &Coord) -> bool {
        self.location.distance(other) <= self.range()
    }

    fn range(&self) -> i64 {
        self.location.distance(&self.closest)
    }
}

fn parse_sensors(file: File) -> Vec<Sensor> {
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut sensor_x: i64 = 0;
            let mut sensor_y: i64 = 0;
            let mut beacon_x: i64 = 0;
            let mut beacon_y: i64 = 0;

            sscanf!(
                line.as_str(),
                "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}",
                sensor_x, sensor_y, beacon_x, beacon_y
            ).unwrap();

            Sensor {
                location: Coord { x: sensor_x, y: sensor_y },
                closest: Coord { x: beacon_x, y: beacon_y },
            }
        })
        .collect()
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open test.txt");

    let sensors = parse_sensors(file);

    let min_x = sensors.iter()
        .map(|sensor| sensor.location.x - sensor.range())
        .min()
        .unwrap();

    let max_x = sensors.iter()
        .map(|sensor| sensor.location.x + sensor.range())
        .max()
        .unwrap();

    let y_to_check = 2000000;
    let num_not_present = (min_x..max_x)
        .map(|x| Coord { x, y: y_to_check })
        .filter(|coord| {
            !sensors.iter().any(|sensor| sensor.closest == *coord) &&
            sensors.iter().any(|sensor| sensor.covers(&coord))
        })
        .count();

    println!("{}", num_not_present);
}
