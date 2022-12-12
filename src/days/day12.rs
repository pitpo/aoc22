extern crate utils;

use std::collections::VecDeque;

use utils::plane::{Boundary, Direction};
use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<Vec<(usize, usize)>>,
    start: usize,
    finish: usize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let line = line.trim();
                line.chars()
                    .map(|c| {
                        if c >= 'a' {
                            (c as usize - 'a' as usize, usize::MAX)
                        } else {
                            (c as usize, usize::MAX)
                        }
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<Vec<(usize, usize)>>>();
        Solver {
            input,
            start: 'S' as usize,
            finish: 'E' as usize,
        }
    }

    fn get_pos(&self, value: usize) -> (usize, usize) {
        for i in 0..self.input.len() {
            for j in 0..self.input[i].len() {
                if self.input[i][j].0 == value {
                    return (i, j);
                }
            }
        }
        return (usize::MAX, usize::MAX);
    }

    fn calculate_distances(
        map: &mut Vec<Vec<(usize, usize)>>,
        start_pos: (usize, usize),
        finish_pos: (usize, usize),
    ) {
        let boundary = Boundary::new_array_boundary(map[0].len(), map.len());
        map[start_pos.0][start_pos.1] = (0, 0);
        map[finish_pos.0][finish_pos.1].0 = 'z' as usize - 'a' as usize;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(start_pos);
        while map[finish_pos.0][finish_pos.1].1 == usize::MAX {
            if queue.is_empty() {
                return;
            }
            let (cur_i, cur_j) = queue.pop_front().unwrap();
            Direction::get_basic_directions().iter().for_each(|dir| {
                if let Some((i, j)) = boundary.move_array_iterator((cur_i, cur_j), dir) {
                    if map[i][j].0 <= map[cur_i][cur_j].0 + 1
                        && map[i][j].1 > map[cur_i][cur_j].1 + 1
                    {
                        map[i][j].1 = map[cur_i][cur_j].1 + 1;
                        queue.push_back((i, j));
                    }
                }
            });
        }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let start_pos = self.get_pos(self.start);
        let finish_pos = self.get_pos(self.finish);
        let mut map = self.input.clone();
        Solver::calculate_distances(&mut map, start_pos, finish_pos);
        let result = map[finish_pos.0][finish_pos.1].1;
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let finish_pos = self.get_pos(self.finish);
        // so basically, 'b's are only in the second column and entire first column is filled with 'a'
        // this quite limits the initial choice
        let starting_positions = self
            .input
            .iter()
            .enumerate()
            .map(|(i, _)| (i, 0))
            .collect::<Vec<(usize, usize)>>();
        let results = starting_positions
            .iter()
            .map(|pos| {
                let mut map = self.input.clone();
                Solver::calculate_distances(&mut map, *pos, finish_pos);
                return map[finish_pos.0][finish_pos.1].1;
            })
            .collect::<Vec<usize>>();
        let result = results.iter().min().unwrap();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "31";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "29";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
