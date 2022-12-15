use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

use scanf::sscanf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Sensor {
    location: Coord,
    range: i32
}

impl Coord {
    #[inline]
    fn distance(&self, other: &Coord) -> i32 {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();

        x + y
    }
}

impl Sensor {
    fn covers(&self, other: &Coord) -> bool {
        self.location.distance(other) <= self.range
    }

    fn get_just_outside_points(&self) -> Vec<Coord> {
        let min_x = self.location.x - self.range - 1;
        let max_x = self.location.x + self.range + 1;
        let min_y = self.location.y - self.range - 1;
        let max_y = self.location.y + self.range + 1;

        let mut points: Vec<Coord> = Vec::new();

        for y in min_y..max_y + 1 {
            let diff = (self.location.y - y).abs();

            if min_x + diff == max_x - diff {
                points.push(Coord { x: min_x + diff, y });
            } else {
                points.push(Coord { x: min_x + diff, y });
                points.push(Coord { x: max_x - diff, y });
            }
        }

        points
    }
}

fn parse_sensors(file: File) -> Vec<Sensor> {
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut sensor_x = 0;
            let mut sensor_y = 0;
            let mut beacon_x = 0;
            let mut beacon_y = 0;

            sscanf!(
                line.as_str(),
                "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                sensor_x, sensor_y, beacon_x, beacon_y
            ).unwrap();

            let sensor = Coord { x: sensor_x, y: sensor_y };
            let beacon = Coord { x: beacon_x, y: beacon_y };

            let distance = sensor.distance(&beacon);
            Sensor {
                location: sensor,
                range: distance
            }
        })
        .collect()
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open test.txt");

    let sensors = parse_sensors(file);

    let max_x = 4000000;
    let max_y = 4000000;

    let possible_beacons: Vec<Coord> = sensors.iter()
        .flat_map(|sensor| sensor.get_just_outside_points())
        .filter(|coord| (coord.x >= 0 && coord.x <= max_x) && (coord.y >= 0 && coord.y <= max_y))
        .collect();

    let distress_beacon = possible_beacons.iter()
        .filter(|coord| !sensors.iter().any(|sensor| sensor.covers(coord)))
        .next()
        .unwrap();

    let tuning_freq: i128 = (i128::from(distress_beacon.x) * 4000000_i128) + i128::from(distress_beacon.y);

    println!("{}", tuning_freq);
}
