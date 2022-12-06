extern crate utils;

use std::collections::HashSet;
use utils::ChallengeSolver;

pub struct Solver {
    input: String,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input.trim();
        Solver {
            input: String::from(input),
        }
    }

    pub fn find_window_with_unique_chars_index(&self, window_size: usize) -> usize {
        self.input
            .chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
            .find_map(|(i, window)| {
                if window.len() == HashSet::<&char>::from_iter::<&[char]>(window).len() {
                    return Some(i + window_size);
                }
                None
            })
            .unwrap()
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.find_window_with_unique_chars_index(4);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = self.find_window_with_unique_chars_index(14);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "11";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "26";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
