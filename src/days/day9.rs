extern crate utils;

use core::panic;
use std::collections::HashSet;

use utils::plane::{Boundary, Direction};
use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<(Direction, isize)>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let (direction, movement) = line.trim().split_once(' ').unwrap();
                let direction = match direction {
                    "R" => Direction::Right,
                    "U" => Direction::Up,
                    "L" => Direction::Left,
                    "D" => Direction::Down,
                    _ => panic!("Invalid input"),
                };
                (direction, movement.parse::<isize>().unwrap())
            })
            .collect::<Vec<(Direction, isize)>>();
        Solver { input }
    }

    fn update_tail(tail_position: &mut (isize, isize), head_position: &(isize, isize)) {
        if head_position.0.abs_diff(tail_position.0) > 1
            || head_position.1.abs_diff(tail_position.1) > 1
        {
            *tail_position = (
                tail_position.0 + (head_position.0 - tail_position.0).signum(),
                tail_position.1 + (head_position.1 - tail_position.1).signum(),
            );
        }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut cur_head_position = (0, 0);
        let mut cur_tail_position = (0, 0);
        let mut visited_spaces: HashSet<(isize, isize)> = HashSet::new();
        let boundary = Boundary::new_infinite_boundary();
        self.input.iter().for_each(|(dir, movement)| {
            for _ in 0..*movement {
                cur_head_position = boundary.move_iterator(cur_head_position, dir).unwrap();
                Solver::update_tail(&mut cur_tail_position, &cur_head_position);
                visited_spaces.insert(cur_tail_position);
            }
        });
        let result = visited_spaces.len();
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut tail_positions = [(0, 0); 10];
        let mut visited_spaces: HashSet<(isize, isize)> = HashSet::new();
        let boundary = Boundary::new_infinite_boundary();
        self.input.iter().for_each(|(dir, movement)| {
            for _ in 0..*movement {
                let head = tail_positions.first().unwrap();
                *tail_positions.first_mut().unwrap() = boundary.move_iterator(*head, dir).unwrap();
                for i in 1..10 {
                    let cur_head_position = *tail_positions.get(i - 1).unwrap();
                    let cur_tail_position = tail_positions.get_mut(i).unwrap();
                    Solver::update_tail(cur_tail_position, &cur_head_position);
                }
                visited_spaces.insert(*tail_positions.last().unwrap());
            }
        });
        let result = visited_spaces.len();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "13";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "1";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example3() {
        let solver = Solver::new(String::from(
            "R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20",
        ));
        let result = "36";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
