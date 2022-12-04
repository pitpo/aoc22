extern crate utils;

use std::collections::HashSet;

use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<(HashSet<u32>, HashSet<u32>)>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let (first_range, second_range) = line.trim().split_once(',').unwrap();
                fn build_into_set(range: &str) -> HashSet<u32> {
                    let (range_start, range_end) = range.split_once('-').unwrap();
                    return HashSet::from_iter(
                        range_start.parse::<u32>().unwrap()..range_end.parse::<u32>().unwrap() + 1,
                    );
                }
                return (build_into_set(first_range), build_into_set(second_range));
            })
            .collect();
        Solver { input }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.input.iter().fold(0, |acc, x| {
            let (set_a, set_b) = x;
            let intersection_size = set_a.intersection(set_b).count();
            if intersection_size == set_a.len() || intersection_size == set_b.len() {
                return acc + 1;
            }
            acc
        });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = self.input.iter().fold(0, |acc, x| {
            let (set_a, set_b) = x;
            let intersection_size = set_a.intersection(set_b).count();
            if intersection_size > 0 {
                return acc + 1;
            }
            acc
        });
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "2";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "4";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
