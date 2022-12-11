extern crate utils;

use utils::plane::{Boundary, Direction};
use utils::ChallengeSolver;

pub struct Solver {
    board: Vec<Vec<u8>>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let dimension_length = input.lines().next().unwrap().trim().len();
        let mut board: Vec<Vec<u8>> = vec![vec![0; dimension_length]; dimension_length];
        input.lines().enumerate().for_each(|(i, line)| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .enumerate()
                .for_each(|(j, tree)| {
                    board[i][j] = tree;
                });
        });
        Solver { board }
    }

    fn is_tree_visible(&self, (i, j): (usize, usize)) -> bool {
        let mut is_tree_visible = true;
        let tree = self.board[i][j];
        let boundary =
            Boundary::new_array_boundary(self.board.first().unwrap().len(), self.board.len());
        Direction::get_basic_directions()
            .iter()
            .take_while(|dir| {
                is_tree_visible = true;
                let (mut cur_i, mut cur_j) = (i, j);
                while let Some((i, j)) = boundary.move_array_iterator((cur_i, cur_j), dir) {
                    if self.board[i][j] >= tree {
                        is_tree_visible = false;
                        break;
                    }
                    (cur_i, cur_j) = (i, j);
                }
                !is_tree_visible
            })
            .count();
        is_tree_visible
    }

    fn get_scenic_score(&self, (i, j): (usize, usize)) -> u32 {
        let tree = self.board[i][j];
        let boundary =
            Boundary::new_array_boundary(self.board.first().unwrap().len(), self.board.len());
        Direction::get_basic_directions()
            .iter()
            .fold(1, |mut acc, dir| {
                let (mut cur_i, mut cur_j) = (i, j);
                let mut visibility = 0;
                while let Some((i, j)) = boundary.move_array_iterator((cur_i, cur_j), dir) {
                    visibility += 1;
                    if self.board[i][j] >= tree {
                        break;
                    }
                    (cur_i, cur_j) = (i, j);
                }
                acc *= visibility;
                acc
            })
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.board.iter().enumerate().fold(0, |acc, (i, row)| {
            acc + row.iter().enumerate().fold(0, |acc, (j, _)| {
                if self.is_tree_visible((i, j)) {
                    acc + 1
                } else {
                    acc
                }
            })
        });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = self
            .board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| self.get_scenic_score((i, j)))
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "30373
        25512
        65332
        33549
        35390",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "21";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "8";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
