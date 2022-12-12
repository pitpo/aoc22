extern crate utils;

use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<Monkey>,
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    ADDITION,
    MULTIPLICATION,
}

#[derive(Debug, Clone)]
struct Monkey {
    item_list: Vec<usize>,
    operation: Operation,
    parameter: usize,
    test_val: usize,
    success_target: usize,
    failure_target: usize,
    inspection_counter: usize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let mut prev_char = '\0';
        let input = input
            .split(|c| {
                let mut result = false;
                if prev_char == '\n' && c == '\n' {
                    result = true;
                }
                prev_char = c;
                result
            })
            .map(|monkey_string| {
                let mut monkey_string = monkey_string.lines();
                monkey_string.next();
                let item_list = monkey_string
                    .next()
                    .unwrap()
                    .split_once(':')
                    .unwrap()
                    .1
                    .split(',')
                    .filter_map(|item| item.trim().parse().ok())
                    .collect::<Vec<usize>>();

                let operation_string = monkey_string
                    .next()
                    .unwrap()
                    .split_once('=')
                    .unwrap()
                    .1
                    .trim()
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>();
                let mut operation = Operation::MULTIPLICATION;
                if operation_string[0] == "+" {
                    operation = Operation::ADDITION;
                }
                let parameter = operation_string[1].parse().unwrap_or(0);

                let mut collect_next_value = || {
                    monkey_string
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<usize>()
                        .unwrap()
                };
                let test_val = collect_next_value();
                let success_target = collect_next_value();
                let failure_target = collect_next_value();
                Monkey {
                    item_list,
                    operation,
                    parameter,
                    test_val,
                    success_target,
                    failure_target,
                    inspection_counter: 0,
                }
            })
            .collect::<Vec<Monkey>>();
        Solver { input }
    }

    fn run_simulation(monkeys: &mut Vec<Monkey>, steps: usize, is_worry_reduced: bool) {
        let mut items = monkeys
            .iter()
            .map(|monkey| monkey.item_list.clone())
            .collect::<Vec<Vec<usize>>>();
        let least_common_multiple = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test_val);
        for _ in 0..steps {
            monkeys.iter_mut().enumerate().for_each(|(i, mut monkey)| {
                items[i].reverse();
                while !items[i].is_empty() {
                    let mut item = items[i].pop().unwrap();
                    if monkey.operation == Operation::ADDITION {
                        if monkey.parameter > 0 {
                            item += monkey.parameter;
                        } else {
                            item += item;
                        }
                    } else {
                        if monkey.parameter > 0 {
                            item *= monkey.parameter;
                        } else {
                            item *= item;
                        }
                    }
                    if is_worry_reduced {
                        item /= 3;
                    } else {
                        item = item % least_common_multiple;
                    }
                    if item % monkey.test_val == 0 {
                        items[monkey.success_target].push(item);
                    } else {
                        items[monkey.failure_target].push(item);
                    }
                    monkey.inspection_counter += 1;
                }
            });
        }
        monkeys.sort_by(|a, b| a.inspection_counter.cmp(&b.inspection_counter));
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut monkeys = self.input.clone();
        Solver::run_simulation(&mut monkeys, 20, true);
        let result = monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, x| acc * x.inspection_counter);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut monkeys = self.input.clone();
        Solver::run_simulation(&mut monkeys, 10000, false);
        let result = monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, x| acc * x.inspection_counter);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "10605";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "2713310158";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
