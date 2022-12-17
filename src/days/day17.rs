extern crate utils;

use std::collections::{VecDeque, HashSet};

use utils::{
    plane::{Boundary, Direction},
    ChallengeSolver,
};

pub struct Solver {
    input: Vec<Direction>,
}

#[derive(Debug, Clone)]
struct Rock {
    points: Vec<(usize, usize)>,
    middle_pos: (usize, usize),
}

impl Rock {
    fn new(spawn_pos_y: usize, spawn_pos_x: usize, variant: usize) -> Rock {
        let mut points = Vec::new();
        match variant {
            0 => {
                for x in 0..4 {
                    points.push((spawn_pos_y, spawn_pos_x + x));
                }
            }
            1 => {
                points.push((spawn_pos_y, spawn_pos_x + 1));
                points.push((spawn_pos_y + 2, spawn_pos_x + 1));
                for x in 0..3 {
                    points.push((spawn_pos_y + 1, spawn_pos_x + x));
                }
            }
            2 => {
                for x in 0..3 {
                    points.push((spawn_pos_y, spawn_pos_x + x));
                }
                for y in 1..3 {
                    points.push((spawn_pos_y + y, spawn_pos_x + 2));
                }
            }
            3 => {
                for y in 0..4 {
                    points.push((spawn_pos_y + y, spawn_pos_x));
                }
            }
            4 => {
                for x in 0..2 {
                    for y in 0..2 {
                        points.push((spawn_pos_y + y, spawn_pos_x + x));
                    }
                }
            }
            _ => {
                panic!("Invalid rock variant");
            }
        }
        let mut rock = Rock {
            points,
            middle_pos: (0, 0),
        };
        rock._update_middle_pos();
        return rock;
    }

    fn _update_middle_pos(&mut self) {
        let pos_sum = self
            .points
            .iter()
            .fold((0, 0), |acc, pos| (acc.0 + pos.0, acc.1 + pos.1));
        self.middle_pos = (pos_sum.0 / self.points.len(), pos_sum.1 / self.points.len());
    }

    fn can_push(&self, boundary: &Boundary, direction: &Direction, rocks: &VecDeque<Rock>) -> bool {
        let can_move = self
            .points
            .iter()
            .filter_map(|point| boundary.move_array_iterator(*point, direction))
            .count()
            == self.points.len();
        if can_move {
            let mut moved_rock = self.clone();
            moved_rock.commit_move(boundary, direction);
            return rocks
                .iter()
                .filter_map(|other_rock| moved_rock._is_colliding(other_rock))
                .next()
                .is_none();
        }
        return false;
    }

    fn commit_move(&mut self, boundary: &Boundary, direction: &Direction) {
        self.points
            .iter_mut()
            .for_each(|point| *point = boundary.move_array_iterator(*point, direction).unwrap());
        self._update_middle_pos();
    }

    fn get_highest_y(&self) -> usize {
        self.points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0
    }

    fn _is_colliding(&self, other: &Rock) -> Option<()> {
        let manhattan_distance = self.middle_pos.0.abs_diff(other.middle_pos.0)
            + self.middle_pos.1.abs_diff(other.middle_pos.1);
        if manhattan_distance < 5 {
            if let Some(_) = self
                .points
                .iter()
                .filter_map(|self_point| {
                    other
                        .points
                        .iter()
                        .filter_map(|other_point| {
                            if self_point == other_point {
                                return Some(());
                            }
                            None
                        })
                        .next()
                })
                .next()
            {
                return Some(());
            }
        }
        None
    }
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .trim()
            .chars()
            .map(|c| {
                if c == '<' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect::<Vec<Direction>>();
        Solver { input }
    }

    fn run_simulation(&self, num_of_rocks: usize) -> usize {
        let mut rocks = VecDeque::new();
        let vec_len = 30;
        rocks.reserve(vec_len);
        let boundary = Boundary::new(0, 1, 7, isize::MAX);
        let mut highest_y = 0;
        let mut wind = self.input.iter().cycle();
        for i in 0..num_of_rocks {
            let mut rock = Rock::new(highest_y + 4, 2, i % 5);
            let mut is_pushed_down = true;
            while is_pushed_down {
                let direction = wind.next().unwrap();
                if rock.can_push(&boundary, direction, &rocks) {
                    rock.commit_move(&boundary, direction);
                }
                if rock.can_push(&boundary, &Direction::Up, &rocks) {
                    rock.commit_move(&boundary, &Direction::Up)
                } else {
                    is_pushed_down = false;
                }
            }
            let rock_highest_y = rock.get_highest_y();
            if rock_highest_y > highest_y {
                highest_y = rock_highest_y;
            }
            rocks.push_front(rock);
            rocks.truncate(vec_len);
        }
        return highest_y;
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let num_of_rocks = 2022;
        let result = self.run_simulation(num_of_rocks);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        // it's super slow, but can probably find solution for any input
        // it should find candidate in max 2 loop iterations, but it will still take a while
        // due to amount of candidates (well, this is literally bruteforce)
        // in sample data the cycle started to appear after going through
        // the wind list 2 times [(2*len)/4], so i sticked to that
        // though it's way too high for actual input
        // decreasing it for real input case doesn't save much time
        // it's still got to iterate over all possible candidates once
        // and it seems that candidates need to be divisible by 5
        // maybe it's a coincidence but that also matches amount of shape variants
        let mut candidates = (5..self.input.len()).filter(|num| num % 5 == 0).collect::<HashSet<usize>>();
        for i in self.input.len()/2..self.input.len() {
            let mut new_candidates = HashSet::new();
            let cycle_offset = self.run_simulation(i);
            for candidate in candidates.clone() {
                let result = self.run_simulation(i + candidate);
                let result2 = self.run_simulation(i + candidate * 2);
                if result2 - result == result - cycle_offset {
                    println!("found possible candidate for {i} {candidate}");
                    new_candidates.insert(candidate);
                }
            }
            candidates = candidates.intersection(&new_candidates).cloned().collect();
            println!("remaining candidates: {candidates:?}");
            let mut are_multiples = true;
            let lowest = candidates.iter().min().unwrap();
            for candidate in candidates.clone() {
                if candidate % lowest != 0 {
                    are_multiples = false;
                }
            }
            if are_multiples {
                break;
            }
            else if candidates.len() == 1 {
                break;
            }
        }
        let cycle_length = candidates.iter().next().unwrap();
        println!("final cycle length: {cycle_length}");
        let before_cycle = self.run_simulation(self.input.len()/2);
        let after_cycle = self.run_simulation(self.input.len()/2 + cycle_length);
        let diff = after_cycle - before_cycle;

        let cycle_remainder = (1000000000000 - self.input.len()/2) % cycle_length;
        let cycled_iters = (1000000000000 - self.input.len()/2) - cycle_remainder;
        let num_of_cycles = cycled_iters / cycle_length;            
        let result = diff * num_of_cycles + self.run_simulation(self.input.len()/2 + cycle_remainder);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>")
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "3068";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "1514285714288";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
