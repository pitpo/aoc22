extern crate utils;

use core::panic;
use std::collections::HashMap;

use utils::ChallengeSolver;

#[derive(Clone)]
enum Operator {
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
}

#[derive(Clone)]
struct Operation {
    lhs: String,
    rhs: String,
    operator: Operator,
}

pub struct Solver {
    numbers: HashMap<String, usize>,
    operations: HashMap<String, Operation>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let mut numbers: HashMap<String, usize> = HashMap::new();
        let mut operations: HashMap<String, Operation> = HashMap::new();
        input.lines().for_each(|line| {
            let line = line.trim();
            let line_split = line.split_whitespace().collect::<Vec<&str>>();
            let mut key = String::from(line_split[0]);
            key.pop();
            if let Ok(num) = line_split[1].parse::<usize>() {
                numbers.insert(key, num);
            } else {
                let operator = match line_split[2] {
                    "+" => Operator::ADDITION,
                    "-" => Operator::SUBTRACTION,
                    "*" => Operator::MULTIPLICATION,
                    "/" => Operator::DIVISION,
                    _ => {
                        panic!("Invalid operator")
                    }
                };
                let operation = Operation {
                    lhs: String::from(line_split[1]),
                    rhs: String::from(line_split[3]),
                    operator,
                };
                operations.insert(key, operation);
            }
        });
        Solver {
            numbers,
            operations,
        }
    }

    fn calc_next_value(
        numbers: &HashMap<String, usize>,
        operations: &HashMap<String, Operation>,
        cur_key: &String,
        cur_humn_value: &usize,
    ) -> Option<(String, usize)> {
        if cur_key == "humn" {
            return None;
        }
        let cur_op = operations.get(cur_key).unwrap();
        let mut next_key = cur_op.lhs.clone();
        let mut is_next_rhs = false;
        let other_val: usize;
        if numbers.contains_key(&cur_op.lhs) && cur_op.lhs != "humn" {
            other_val = *numbers.get(&cur_op.lhs).unwrap();
            next_key = cur_op.rhs.clone();
            is_next_rhs = true;
        } else {
            other_val = *numbers.get(&cur_op.rhs).unwrap();
        }
        let next_humn_value = match cur_op.operator {
            Operator::ADDITION => cur_humn_value - other_val,
            Operator::SUBTRACTION => {
                if is_next_rhs {
                    other_val - cur_humn_value
                } else {
                    cur_humn_value + other_val
                }
            }
            Operator::MULTIPLICATION => cur_humn_value / other_val,
            Operator::DIVISION => cur_humn_value * other_val,
        };
        return Some((next_key, next_humn_value));
    }

    fn calculate_numbers(
        numbers: &mut HashMap<String, usize>,
        operations: &HashMap<String, Operation>,
        skip_humn: bool,
    ) {
        let mut ops_done = usize::MAX;
        while ops_done != 0 {
            ops_done = 0;
            operations.iter().for_each(|(key, op)| {
                if !numbers.contains_key(key)
                    && numbers.contains_key(&op.lhs)
                    && numbers.contains_key(&op.rhs)
                    && (!skip_humn || (skip_humn && op.lhs != "humn" && op.rhs != "humn"))
                {
                    let result = match op.operator {
                        Operator::ADDITION => {
                            numbers.get(&op.lhs).unwrap() + numbers.get(&op.rhs).unwrap()
                        }
                        Operator::SUBTRACTION => {
                            numbers.get(&op.lhs).unwrap() - numbers.get(&op.rhs).unwrap()
                        }
                        Operator::MULTIPLICATION => {
                            numbers.get(&op.lhs).unwrap() * numbers.get(&op.rhs).unwrap()
                        }
                        Operator::DIVISION => {
                            numbers.get(&op.lhs).unwrap() / numbers.get(&op.rhs).unwrap()
                        }
                    };
                    numbers.insert(key.clone(), result);
                    ops_done += 1;
                }
            });
        }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut numbers = self.numbers.clone();
        let operations = self.operations.clone();
        Solver::calculate_numbers(&mut numbers, &operations, false);
        let root_key = String::from("root");
        let result = numbers.get(&root_key).unwrap();
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut numbers = self.numbers.clone();
        let operations = self.operations.clone();
        Solver::calculate_numbers(&mut numbers, &operations, true);
        let root_key = String::from("root");
        let mut next_key: String;
        let mut real_humn_value: usize;
        if numbers.contains_key(&operations.get(&root_key).unwrap().lhs) {
            next_key = operations.get(&root_key).unwrap().rhs.clone();
            real_humn_value = *numbers
                .get(&operations.get(&root_key).unwrap().lhs)
                .unwrap();
        } else {
            next_key = operations.get(&root_key).unwrap().lhs.clone();
            real_humn_value = *numbers
                .get(&operations.get(&root_key).unwrap().rhs)
                .unwrap();
        }
        while let Some(data) =
            Solver::calc_next_value(&numbers, &operations, &next_key, &real_humn_value)
        {
            real_humn_value = data.1;
            next_key = data.0;
        }

        let result = real_humn_value;
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "152";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "301";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
