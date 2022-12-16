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
    room_to_vec_pos_map: HashMap<usize, usize>,
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
        let mut room_to_vec_pos_map = HashMap::new();
        let mut input = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let mut line = line.trim().split(&[' ', '=', ',', ';']);
                let room_name = line.nth(1).unwrap();
                let room = *room_name_map.get(room_name).unwrap();
                room_to_vec_pos_map.insert(room, i);
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
            room_to_vec_pos_map,
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
}

#[derive(Copy, Clone)]
struct Explorer<'a> {
    room: &'a Node,
    pressure_level: usize,
    total_pressure: usize,
    time: usize,
}

struct Savepoint<'a> {
    elf: Explorer<'a>,
    elephant: Explorer<'a>,
    visited: HashSet<usize>,
}

impl<'a> Explorer<'a> {
    fn new(first_room: &'a Node) -> Explorer<'a> {
        Explorer {
            room: first_room,
            pressure_level: 0,
            total_pressure: 0,
            time: 0,
        }
    }

    fn explore(
        &self,
        next_room: &'a Node,
        time_limit: &usize,
        visited: &HashSet<usize>,
    ) -> Option<Explorer<'a>> {
        if self.room.distances.contains_key(&next_room.room)
            && self.time + self.room.distances[&next_room.room] < *time_limit
            && !visited.contains(&next_room.room)
        {
            let total_pressure = self.total_pressure
                + (1 + self.room.distances[&next_room.room]) * self.pressure_level;
            let pressure_level = self.pressure_level + next_room.flow_rate;
            let time = self.time + 1 + self.room.distances[&next_room.room];
            return Some(Explorer {
                room: next_room,
                pressure_level,
                total_pressure,
                time,
            });
        }
        None
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let time_limit = 30;
        let mut visited = HashSet::new();
        visited.reserve(self.input[self.first_node].distances.len());
        visited.insert(self.first_node);
        let explorer = Explorer::new(&self.input[self.room_to_vec_pos_map[&self.first_node]]);
        let initial_state = Savepoint {
            elf: explorer,
            elephant: explorer,
            visited,
        };
        let mut stack = vec![initial_state];
        let mut max_possible_pressure = 0;
        stack.reserve(10000);
        while !stack.is_empty() {
            let current_state = stack.pop().unwrap();
            for i in 0..self.input.len() {
                let next_room = &self.input[i];
                if let Some(elf) =
                    current_state
                        .elf
                        .explore(&next_room, &time_limit, &current_state.visited)
                {
                    let mut visited = current_state.visited.clone();
                    visited.insert(next_room.room);
                    stack.push(Savepoint {
                        elf,
                        elephant: current_state.elephant,
                        visited,
                    });
                    let possible_end_pressure =
                        elf.total_pressure + elf.pressure_level * (time_limit - elf.time);
                    if possible_end_pressure > max_possible_pressure {
                        max_possible_pressure = possible_end_pressure;
                    }
                }
            }
        }
        String::from(max_possible_pressure.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let time_limit = 26;
        let mut visited = HashSet::new();
        visited.reserve(self.input[self.first_node].distances.len());
        visited.insert(self.first_node);
        let explorer = Explorer::new(&self.input[self.room_to_vec_pos_map[&self.first_node]]);
        let initial_state = Savepoint {
            elf: explorer,
            elephant: explorer,
            visited,
        };
        let mut stack = vec![initial_state];
        let mut max_possible_pressure = 0;
        stack.reserve(10000);
        while !stack.is_empty() {
            let current_state = stack.pop().unwrap();
            for i in 0..self.input.len() {
                let next_room = &self.input[i];
                if let Some(elf) =
                    current_state
                        .elf
                        .explore(&next_room, &time_limit, &current_state.visited)
                {
                    let mut visited = current_state.visited.clone();
                    visited.insert(next_room.room);
                    for j in 0..self.input.len() {
                        let next_elephant_room = &self.input[j];
                        let mut visited = visited.clone();
                        if let Some(elephant) = current_state.elephant.explore(
                            &next_elephant_room,
                            &time_limit,
                            &visited,
                        ) {
                            visited.insert(next_elephant_room.room);
                            stack.push(Savepoint {
                                elf,
                                elephant,
                                visited,
                            });
                            let possible_end_pressure = elf.total_pressure
                                + elf.pressure_level * (time_limit - elf.time)
                                + elephant.total_pressure
                                + elephant.pressure_level * (time_limit - elephant.time);
                            if possible_end_pressure > max_possible_pressure {
                                max_possible_pressure = possible_end_pressure;
                            }
                        }
                    }
                }
            }
        }
        String::from(max_possible_pressure.to_string())
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
