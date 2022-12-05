extern crate utils;

use utils::ChallengeSolver;

pub struct Solver {
    moves: Vec<Vec<usize>>,
    stacks: Vec<Vec<char>>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input.split_once("\n\n").unwrap();
        let moves = input
            .1
            .lines()
            .map(|line| {
                return line
                    .trim()
                    .split_whitespace()
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect::<Vec<usize>>();
            })
            .collect::<Vec<Vec<usize>>>();
        // yada yada won't work when there is no move to last column, whatevs
        let columns = moves
            .iter()
            .max_by(|x, y| x.last().unwrap().cmp(y.last().unwrap()))
            .unwrap()
            .last()
            .unwrap();
        let mut stacks: Vec<Vec<char>> = vec![vec![]; *columns];
        input.0.lines().rev().skip(1).for_each(|line| {
            // ok this is possibly the most retarded way of doing this
            let mut column = 0;
            line.chars().enumerate().for_each(|(i, char)| {
                if (i + 3) % 4 == 0 {
                    column += 1;
                }
                if !char.is_alphabetic() {
                    return;
                }
                stacks.get_mut(column - 1).unwrap().push(char);
            })
        });
        Solver { moves, stacks }
    }

    fn get_result_string(stacks: &Vec<Vec<char>>) -> String {
        stacks.iter().fold(String::new(), |mut acc, stack| {
            if !stack.is_empty() {
                acc.push(*stack.last().unwrap());
            }
            acc
        })
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut stacks = self.stacks.clone();
        self.moves.iter().for_each(|move_set| {
            let (amount, from, to) = (move_set[0], move_set[1] - 1, move_set[2] - 1);
            for _ in 0..amount {
                let c = stacks.get_mut(from).unwrap().pop().unwrap();
                stacks.get_mut(to).unwrap().push(c);
            }
        });
        let result = Solver::get_result_string(&stacks);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut stacks = self.stacks.clone();
        self.moves.iter().for_each(|move_set| {
            let (amount, from, to) = (move_set[0], move_set[1] - 1, move_set[2] - 1);
            let stack_from: &mut Vec<char> = stacks.get_mut(from).unwrap();
            let split_point = stack_from.len() - amount;
            let mut pickup = stack_from.split_off(split_point);
            stacks.get_mut(to).unwrap().append(&mut pickup);
        });
        let result = Solver::get_result_string(&stacks);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "CMZ";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "MCD";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
