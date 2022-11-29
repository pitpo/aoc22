extern crate utils;

use utils::Day;

pub struct Day1 {
    input: Vec<u32>,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        let input = input
            .lines()
            .map(|line| line.trim().parse::<u32>().unwrap())
            .collect();
        Day1 { input }
    }

    fn get_window_fold(&self, vec: &Vec<u32>) -> u32 {
        vec.windows(2).fold(0, |acc, x| {
            if x[0] < x[1] {
                return acc + 1;
            };
            return acc;
        })
    }
}

impl Day for Day1 {
    fn get_part_a_result(&self) -> String {
        let result = self.get_window_fold(self.input.as_ref());
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let windows_summed = self.input.windows(3).map(|x| x.iter().sum()).collect::<Vec<u32>>();
        let result = self.get_window_fold(windows_summed.as_ref());
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from("199
        200
        208
        210
        200
        207
        240
        269
        260
        263")
    }

    #[test]
    fn example1() {
        let solver = Day1::new(get_input());
        let result = "7";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Day1::new(get_input());
        let result = "5";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}