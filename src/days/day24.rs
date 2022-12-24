extern crate utils;

use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use utils::{
    plane::{Boundary, Direction},
    ChallengeSolver,
};

pub struct Solver {
    input: Vec<Blizzard>,
    boundary: Boundary,
    destination: (usize, usize),
}

#[derive(Clone, Copy)]
struct Blizzard {
    pos: (usize, usize),
    dir: Direction,
}

#[derive(Debug, Hash, Clone, Copy)]
struct Savepoint {
    player_pos: (isize, isize),
    time: usize,
}

impl Savepoint {
    fn get_hash(&self, time_cycle: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut savepoint = self.clone();
        savepoint.time = savepoint.time % time_cycle;
        savepoint.hash(&mut hasher);
        hasher.finish()
    }
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let y_size = input.lines().count() - 2;
        let x_size = input.lines().nth(0).unwrap().len() - 2;
        let destination = (y_size - 1, x_size - 1);
        let boundary = Boundary::new_array_boundary(x_size, y_size);
        let input = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        let pos = (y - 1, x - 1);
                        match c {
                            '>' => Some(Blizzard {
                                pos,
                                dir: Direction::Right,
                            }),
                            'v' => Some(Blizzard {
                                pos,
                                dir: Direction::Down,
                            }),
                            '<' => Some(Blizzard {
                                pos,
                                dir: Direction::Left,
                            }),
                            '^' => Some(Blizzard {
                                pos,
                                dir: Direction::Up,
                            }),
                            _ => None,
                        }
                    })
                    .collect::<Vec<Blizzard>>()
            })
            .collect::<Vec<Blizzard>>();
        Solver {
            input,
            boundary,
            destination,
        }
    }

    fn run_simulation(&self, is_part_a: bool) -> usize {
        let mut blizzards = self.input.clone();
        let mut current_time = 0;
        let possible_directions = [
            Direction::None,
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let cycle = (self.destination.0 + 1) * (self.destination.1 + 1);
        let move_blizzards = |blizzards: &mut Vec<Blizzard>| {
            blizzards.iter_mut().for_each(|bliz| {
                if let Some(pos) = self.boundary.move_array_iterator(bliz.pos, &bliz.dir) {
                    bliz.pos = pos;
                } else {
                    bliz.pos = self.boundary.wrap_array_iterator(bliz.pos, &bliz.dir);
                }
            });
        };
        let mut get_shortest_path = |destination: (usize, usize), start_pos: (isize, isize)| {
            let mut queue: VecDeque<Savepoint> = VecDeque::new();
            let mut advance_time = true;
            let mut seen: HashSet<u64> = HashSet::new();
            queue.push_back(Savepoint {
                player_pos: start_pos,
                time: 0,
            });
            'advance_time: while advance_time {
                current_time += 1;
                move_blizzards(&mut blizzards);

                while let Some(savepoint) = queue.pop_front() {
                    if savepoint.time == current_time || !advance_time {
                        queue.push_front(savepoint);
                        continue 'advance_time;
                    }
                    possible_directions.iter().for_each(|dir| {
                        if let Some(pos) = self.boundary.move_iterator(savepoint.player_pos, dir) {
                            let pos_u = (pos.0 as usize, pos.1 as usize);
                            let is_move_possible = blizzards
                                .iter()
                                .filter_map(|bliz| {
                                    if pos_u == bliz.pos {
                                        return Some(());
                                    }
                                    None
                                })
                                .next()
                                .is_none();
                            if is_move_possible {
                                if pos_u == destination {
                                    advance_time = false;
                                }
                                let savepoint = Savepoint {
                                    player_pos: pos,
                                    time: current_time,
                                };
                                let hash = savepoint.get_hash(cycle);
                                if !seen.contains(&hash) {
                                    seen.insert(hash);
                                    queue.push_back(savepoint);
                                }
                            }
                        }
                    });
                }
            }
            current_time += 1;
            move_blizzards(&mut blizzards);
        };
        if is_part_a {
            let start_pos = (-1, 0);
            get_shortest_path(self.destination, start_pos);
        } else {
            let top_left_start = (-1, 0);
            let top_left_dest = (0, 0);
            let bot_right_start = (self.destination.0 as isize + 1, self.destination.1 as isize);
            get_shortest_path(self.destination, top_left_start);
            get_shortest_path(top_left_dest, bot_right_start);
            get_shortest_path(self.destination, top_left_start);
        }

        current_time
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.run_simulation(true);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = self.run_simulation(false);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "#.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "18";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "54";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
