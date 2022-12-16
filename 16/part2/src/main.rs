use std::{collections::{HashMap, HashSet}, fs::File, io::{BufReader, BufRead}, cmp::max};

use pathfinding::prelude::dijkstra_all;
use scanf::sscanf;

#[derive(Debug, Clone)]
struct Room {
    outgoing: HashMap<String, u8>,
    incoming: HashMap<String, u8>,
    valve_weight: Option<u8>,
    shortest_paths: HashMap<String, u64>,
}

const TIME_BUDGET: u64 = 26;

fn parse_rooms(input: File) -> HashMap<String, Room> {
    let mut rooms = HashMap::new();

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut name = String::new();
        let mut flow_rate: u8 = 0;

        let halves: Vec<&str> = line.split(";").collect();

        let parsable_half = halves[0];

        sscanf!(parsable_half, "Valve {string} has flow rate={u8}", name, flow_rate).unwrap();

        let connections: Vec<String> = halves[1].split_whitespace()
            .skip(4)
            .map(|s| s.replace(",", ""))
            .collect();

        let mut routes: HashMap<String, u8> = HashMap::new();

        for connection in connections {
            routes.insert(connection, 1);
        }

        let room = Room {
            outgoing: routes,
            incoming: HashMap::new(),
            valve_weight: if flow_rate == 0 { None } else { Some(flow_rate) },
            shortest_paths: HashMap::new(),
        };

        rooms.insert(name, room);
    }

    // Find incoming connections
    let room_names = rooms.keys().cloned().collect::<Vec<String>>();

    for name in room_names {
        let room = rooms.get_mut(&name).unwrap().clone();
        let outgoing = room.outgoing.clone();

        for (destination, weight) in outgoing {
            let destination_room = rooms.get_mut(&destination).unwrap();

            destination_room.incoming.insert(name.clone(), weight);
        }
    }

    rooms
}

fn eliminate_useless_rooms(rooms: &mut HashMap<String, Room>) {
    let useless_room_names: Vec<String> = rooms.iter()
        .filter(|(_, room)| room.valve_weight.is_none())
        .filter(|(name, _)| name.as_str() != "AA")
        .map(|(name, _)| name.clone())
        .collect();

    for to_remove in useless_room_names {
        let room = rooms.remove(&to_remove).unwrap();

        for (origin, origin_weight) in &room.incoming {
            let mut origin_room = rooms.get(origin).unwrap().clone();

            origin_room.outgoing.remove(&to_remove);

            for (destination, weight) in &room.outgoing {
                let destination_room = rooms.get_mut(destination).unwrap();

                destination_room.incoming.remove(&to_remove);

                if origin != destination {
                    origin_room.outgoing.insert(destination.clone(), origin_weight + weight);
                    destination_room.incoming.insert(origin.clone(), origin_weight + weight);
                }
            }

            rooms.insert(origin.clone(), origin_room);
        }


    }
}

fn determine_shortest_paths(rooms: &mut HashMap<String, Room>) {
    let room_names = rooms.keys().cloned().collect::<Vec<String>>();

    for from in room_names.iter() {
        let paths: HashMap<String, (String, u64)> = dijkstra_all(from, |node| {
            let room = rooms.get(node).unwrap();

            room.outgoing.iter().map(|(to, weight)| (to.clone(), *weight as u64))
        });

        let shortest_paths: HashMap<String, u64> = paths.iter()
            .map(|path| (path.0.clone(), path.1.1))
            .collect();

        let room = rooms.get_mut(from).unwrap();
        room.shortest_paths = shortest_paths;
    }
}

fn get_next_iteration(prev_iter: &[(Route, Route)], rooms: &HashMap<String, Room>) -> Vec<(Route, Route)> {
    let mut next_iter: Vec<(Route, Route)> = Vec::new();
    let mut found_routes: HashSet<(Route, Route)> = HashSet::new();

    for (route_you, route_elephant) in prev_iter.iter() {
        let remaining_rooms_for_you: Vec<&String> = rooms.keys()
            .filter(|name| !route_you.0.contains(name))
            .filter(|name| !route_elephant.0.contains(name))
            .collect();

        for your_room in remaining_rooms_for_you.iter() {
            let rooms_for_elephant: Vec<&String> = remaining_rooms_for_you.clone().iter()
                .filter(|name| *name != your_room)
                .copied()
                .collect();

            for elephant_room in rooms_for_elephant.iter() {
                let weight_you = route_you.1 + rooms.get(route_you.0.last().unwrap()).unwrap().shortest_paths[*your_room] + 1;
                let weight_elephant = route_elephant.1 + rooms.get(route_elephant.0.last().unwrap()).unwrap().shortest_paths[*elephant_room] + 1;

                if weight_you <= TIME_BUDGET && weight_elephant <= TIME_BUDGET {
                    let mut new_route_you = route_you.0.clone();
                    new_route_you.push((*your_room).clone());

                    let mut new_route_elephant = route_elephant.0.clone();
                    new_route_elephant.push((*elephant_room).clone());

                    let new_you = (new_route_you, weight_you);
                    let new_elephant = (new_route_elephant, weight_elephant);

                    if !found_routes.contains(&(new_elephant.clone(), new_you.clone())) {
                        found_routes.insert((new_you.clone(), new_elephant.clone()));
                        next_iter.push((new_you, new_elephant));
                    }
                }
            }
        }
    }

    next_iter
}

type Route = (Vec<String>, u64);

fn get_all_viable_routes(rooms: &HashMap<String, Room>) -> Vec<(Vec<String>, Vec<String>)> {
    // let names = rooms.keys().cloned().collect::<Vec<String>>();

    let mut all_iters: Vec<(Route, Route)> =
        vec![(
            (vec!["AA".to_string()], 0),
            (vec!["AA".to_string()], 0)
        )];

    let mut prev_iter = all_iters.clone();

    let mut i = 0;
    loop {
        println!("Iter {}", i);
        prev_iter = get_next_iteration(&prev_iter, rooms);

        if prev_iter.len() == 0 {
            break;
        }

        println!("Returned {} routes", prev_iter.len());

        all_iters.extend(prev_iter.clone());
        i += 1;
    }

    all_iters.iter()
    .map(|route| (route.0.0.to_owned(), route.1.0.to_owned()))
    .collect()
}

fn get_score_for_route(route: &(Vec<String>, Vec<String>), rooms: &HashMap<String, Room>) -> u64 {
    let mut score: u64 = 0;

    let mut time_spent = 0;

    let mut events = [Option::<u8>::None; (TIME_BUDGET + 1) as usize];

    for i in 0..route.0.len() {
        let room = rooms.get(&route.0[i]).unwrap();

        match room.valve_weight {
            Some(weight) => {
                time_spent += 1;

                events[time_spent as usize] = match events[time_spent as usize] {
                    Some(val) => Some(val + weight),
                    None => Some(weight)
                };
            },
            None => ()
        };

        if i < route.0.len() - 1 {
            time_spent += room.shortest_paths[&route.0[i + 1]];
        }
    }

    time_spent = 0;

    for i in 0..route.1.len() {
        let room = rooms.get(&route.1[i]).unwrap();

        match room.valve_weight {
            Some(weight) => {
                time_spent += 1;

                events[time_spent as usize] = match events[time_spent as usize] {
                    Some(val) => Some(val + weight),
                    None => Some(weight)
                };
            },
            None => ()
        };

        if i < route.1.len() - 1 {
            time_spent += room.shortest_paths[&route.1[i + 1]];
        }
    }

    let mut score_per_turn: u64 = 0;

    for i in 0..TIME_BUDGET {
        match events[i as usize] {
            Some(val) => {
                score_per_turn += val as u64;
            },
            None => ()
        }

        score += score_per_turn;
    }

    score
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut rooms = parse_rooms(file);

    eliminate_useless_rooms(&mut rooms);
    determine_shortest_paths(&mut rooms);

    println!("Getting routes");
    let routes = get_all_viable_routes(&rooms);

    // println!("{:#?}", routes);

    println!("Getting scores");
    let routes_with_score = routes.iter()
        // .map(|route| (get_score_for_route(&route, &rooms), route))
        .map(|route| get_score_for_route(route, &rooms))
        .max()
        .unwrap();

    println!("{:#?}", routes_with_score);
}
