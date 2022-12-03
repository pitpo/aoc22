extern crate utils;

use utils::ChallengeSolver;

pub struct Solver {
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input;
        Solver { }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = "IMPLEMENT ME";
        String::from(result.to_string())
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
        String::from("")
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}