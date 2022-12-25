extern crate utils;

use core::panic;

use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<isize>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| Solver::decode_snafu(line.trim()))
            .collect::<Vec<isize>>();
        Solver { input }
    }

    fn translate_char(c: char) -> isize {
        match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => {
                panic!("ey man, where'd you find that");
            }
        }
    }

    fn translate_num(num: isize) -> char {
        match num {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => {
                panic!("got {num} in translation attempt");
            }
        }
    }

    fn encode_snafu(mut num: isize) -> String {
        let mut snafu = String::new();
        while num > 0 {
            let rem = num % 5;
            num = num / 5;
            match rem {
                0 | 1 | 2 => {
                    snafu.push(Solver::translate_num(rem));
                }
                3 => {
                    num += 1;
                    snafu.push(Solver::translate_num(-2));
                }
                4 => {
                    num += 1;
                    snafu.push(Solver::translate_num(-1));
                }
                _ => {
                    panic!("unreachable");
                }
            }
        }
        snafu.chars().rev().collect::<String>()
    }

    fn decode_snafu(num: &str) -> isize {
        let num_len = num.len() as u32;
        let num = num.chars().enumerate().fold(0, |acc, (i, c)| {
            let base: isize = 5;
            let mult = base.pow(num_len - i as u32 - 1);
            acc + Solver::translate_char(c) * mult
        });
        num
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let sum: isize = self.input.iter().sum();
        println!("{sum}");
        let result = Solver::encode_snafu(sum);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = "MERRY CHRISTMAS";
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "2=-1=0";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "MERRY CHRISTMAS";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
