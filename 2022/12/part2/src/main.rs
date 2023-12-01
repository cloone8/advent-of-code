use std::{fs::File, io::{BufRead, BufReader}, collections::{VecDeque, HashSet}};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Square {
    Start,
    Middle(u8),
    End,
}

type Point = (usize, usize);

impl From<char> for Square {
    fn from(c: char) -> Self {
        match c {
            'S' | 'a' => Square::Start,
            'E' => Square::End,
            _ => Square::Middle(c as u8 - 0x61),
        }
    }
}

impl From<&Square> for u8 {
    fn from(s: &Square) -> Self {
        match s {
            Square::Start => 0,
            Square::End => 25,
            Square::Middle(c) => *c
        }
    }
}

fn parse_grid(input: impl BufRead) -> (Vec<Vec<Square>>, (usize, usize), Vec<Point>) {
    let mut grid = Vec::new();
    let mut row_idx = 0;
    let mut start: Vec<Point> = Vec::new();

    for line in input.lines() {
        let mut col_idx = 0;

        let line = line.unwrap();
        let mut row = Vec::new();

        for c in line.chars() {
            let square = Square::from(c);

            if square == Square::Start {
                start.push((row_idx, col_idx));
            }

            row.push(Square::from(c));
            col_idx += 1;
        }

        grid.push(row);
        row_idx += 1;
    }

    let dims = (grid.len(), grid[0].len());

    (grid, dims, start)
}

fn do_search(
    grid: &Vec<Vec<Square>>,
    grid_dims: (usize, usize),
    queue: &mut VecDeque<(Point, u64)>,
    visited: &mut HashSet<Point>,
    cur: (Point, u64)
) -> Option<u64> {

    let my_coords: Point = cur.0;
    let my_length = cur.1;
    let my_square = &grid[my_coords.0][my_coords.1];
    let my_height = u8::from(my_square);
    let my_max_step = my_height + 1;

    let mut neighbors: Vec<Point> = Vec::new();

    if my_coords.0 > 0 {
        neighbors.push((my_coords.0 - 1, my_coords.1));
    }

    if my_coords.0 < grid_dims.0 - 1 {
        neighbors.push((my_coords.0 + 1, my_coords.1));
    }

    if my_coords.1 > 0 {
        neighbors.push((my_coords.0, my_coords.1 - 1));
    }

    if my_coords.1 < grid_dims.1 - 1 {
        neighbors.push((my_coords.0, my_coords.1 + 1));
    }

    for (row, col) in neighbors {
        let neighbor = &grid[row][col];
        let neighbor_height = u8::from(neighbor);

        if neighbor_height <= my_max_step && !visited.contains(&(row, col)) {
            match neighbor {
                Square::Start => panic!("Start square found in middle of grid!"),
                Square::Middle(_) => {
                    queue.push_back(((row, col), my_length + 1));
                    visited.insert((row, col));
                },
                Square::End => return Some(my_length + 1),
            }
        }
    }

    None
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };

    let reader = BufReader::new(file);

    let (grid, dims, start) = parse_grid(reader);
    let mut queue: VecDeque<(Point, u64)> = start.clone().into_iter().map(|p| (p, 0)).collect();
    let mut visited: HashSet<Point> = start.clone().into_iter().collect();

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();

        let shortest_path = do_search(&grid, dims, &mut queue, &mut visited, cur);

        if shortest_path.is_some() {
            println!("Shortest path: {}", shortest_path.unwrap());
            return;
        }
    }
}
