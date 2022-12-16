extern crate utils;

use std::collections::{HashMap, HashSet};

use utils::ChallengeSolver;

struct Node {
    room: usize,
    connections: Vec<usize>,
    flow_rate: usize,
    distances: HashMap<usize, usize>,
}

pub struct Solver {
    input: Vec<Node>,
    first_node: usize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let num_of_rooms = input.lines().count();
        let room_name_map = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let room_name = String::from(line.trim().split_whitespace().nth(1).unwrap());
                (room_name, i)
            })
            .collect::<HashMap<String, usize>>();
        println!("{room_name_map:#?}");
        let mut input = input
            .lines()
            .map(|line| {
                let mut line = line.trim().split(&[' ', '=', ',', ';']);
                let room_name = line.nth(1).unwrap();
                let room = *room_name_map.get(room_name).unwrap();
                let flow_rate = line.nth(3).unwrap().parse::<usize>().unwrap();
                let tunnels = line
                    .skip(5)
                    .filter_map(|tunnel| {
                        if tunnel.is_empty() {
                            return None;
                        }
                        Some(*room_name_map.get(tunnel).unwrap())
                    })
                    .collect::<Vec<usize>>();
                let mut distances = HashMap::new();
                distances.reserve(num_of_rooms);
                Node {
                    room,
                    flow_rate,
                    connections: tunnels,
                    distances,
                }
            })
            .collect::<Vec<Node>>();
        Solver::calculate_distances(&mut input);
        Solver::remove_irrelevant_distances(&mut input);
        Solver {
            input,
            first_node: *room_name_map.get("AA").unwrap(),
        }
    }

    fn init_distances(nodes: &mut Vec<Node>) {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                let j_room = nodes[j].room.clone();
                let mut distance = usize::MAX / 2; // avoid overflow
                if nodes[i].connections.contains(&j_room) {
                    distance = 1
                }
                nodes[i].distances.insert(j_room, distance);
            }
        }
    }

    fn calculate_distances(nodes: &mut Vec<Node>) {
        Solver::init_distances(nodes);
        for k in 0..nodes.len() {
            let k_room = nodes[k].room;
            for i in 0..nodes.len() {
                for j in 0..nodes.len() {
                    let j_room = nodes[j].room;
                    let cur_dist = nodes[i].distances[&j_room];
                    let calc_dist = nodes[i].distances[&k_room] + nodes[k].distances[&j_room];
                    *nodes[i].distances.get_mut(&j_room).unwrap() =
                        std::cmp::min(cur_dist, calc_dist);
                }
            }
        }
    }

    fn remove_irrelevant_distances(nodes: &mut Vec<Node>) {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if nodes[j].flow_rate == 0 || i == j {
                    let j_room = nodes[j].room;
                    nodes[i].distances.remove(&j_room);
                }
            }
        }
    }

    fn get_room(&self, name: &usize) -> &Node {
        // i f'd up, input should've been map, but it's too late now
        self.input.iter().find(|node| node.room == *name).unwrap()
    }
}

#[derive(Debug)]
struct Savepoint {
    room: usize,
    pressure_level: usize,
    total_pressure: usize,
    time: usize,
    visited: HashSet<usize>,
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let time_limit = 30;
        let initial_state = Savepoint {
            room: self.first_node,
            pressure_level: 0,
            total_pressure: 0,
            time: 0,
            visited: HashSet::new(),
        };
        let mut stack: Vec<Savepoint> = vec![initial_state];
        stack.reserve(1000);
        let mut max_possible_pressure = 0;
        while !stack.is_empty() {
            let current_state = stack.pop().unwrap();
            let current_room = self.get_room(&current_state.room);
            let current_room_name = current_room.room;
            for i in 0..self.input.len() {
                let next_room = self.input[i].room;
                if current_room.distances.contains_key(&next_room)
                    && current_state.time + current_room.distances[&next_room] < time_limit
                    && !current_state.visited.contains(&next_room)
                {
                    let total_pressure = current_state.total_pressure
                        + (1 + current_room.distances[&next_room]) * current_state.pressure_level;
                    let pressure_level =
                        current_state.pressure_level + self.get_room(&next_room).flow_rate;
                    let time = current_state.time + 1 + current_room.distances[&next_room];
                    let mut visited = current_state.visited.clone();
                    visited.insert(current_room_name);
                    stack.push(Savepoint {
                        room: next_room,
                        total_pressure,
                        pressure_level,
                        time,
                        visited,
                    });
                    let possible_end_pressure =
                        total_pressure + pressure_level * (time_limit - time);
                    if possible_end_pressure > max_possible_pressure {
                        max_possible_pressure = possible_end_pressure;
                    }
                }
            }
        }

        String::from(max_possible_pressure.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = "IMPLEMENT ME";
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "1651";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "1707";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
