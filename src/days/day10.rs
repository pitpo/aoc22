extern crate utils;

use std::vec;

use utils::ChallengeSolver;

#[derive(Clone, Copy, Debug)]
enum Operation {
    NOOP,
    ADDX,
}

pub struct Solver {
    input: Vec<(Operation, isize)>,
}

struct CPU {
    x: isize,
    clock: usize,
    breakpoint: usize,
    operation: Option<Operation>,
    operation_time: usize,
    operation_parameter: isize,
    crt: Vec<Vec<char>>,
}

struct BreakpointTriggered {
    x: isize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let mut instruction = line.trim().split_whitespace();
                match instruction.next().unwrap() {
                    "noop" => (Operation::NOOP, 0),
                    "addx" => (
                        Operation::ADDX,
                        instruction.next().unwrap().parse::<isize>().unwrap(),
                    ),
                    _ => panic!("Invalid operation"),
                }
            })
            .collect::<Vec<(Operation, isize)>>();
        Solver { input }
    }
}

impl CPU {
    fn new(breakpoint: usize) -> CPU {
        CPU {
            x: 1,
            clock: 0,
            breakpoint: breakpoint,
            operation: None,
            operation_time: 0,
            operation_parameter: 0,
            crt: vec![vec!['.'; 40]; 6],
        }
    }

    fn is_operation_in_progress(&self) -> bool {
        return self.operation.is_some();
    }

    fn get_operation_length(&self) -> usize {
        match self.operation {
            Some(Operation::NOOP) => 1,
            Some(Operation::ADDX) => 2,
            None => 0,
        }
    }

    fn finish_operation(&mut self) {
        match self.operation {
            Some(Operation::NOOP) => {}
            Some(Operation::ADDX) => self.x += self.operation_parameter,
            None => panic!("No operation running"),
        }
        self.operation = None;
        self.operation_time = 0;
    }

    fn read_operation(&mut self, op: Operation, parameter: isize) {
        self.operation = Some(op);
        self.operation_parameter = parameter;
    }

    fn next_cycle(&mut self) -> Option<BreakpointTriggered> {
        if self.x.abs_diff((self.clock % 40) as isize) < 2 {
            *self
                .crt
                .get_mut(self.clock / 40)
                .unwrap()
                .get_mut(self.clock % 40)
                .unwrap() = '#';
        }
        self.clock += 1;
        self.operation_time += 1;
        let mut breakpoint_triggered = None;
        if self.clock == self.breakpoint {
            breakpoint_triggered = Some(BreakpointTriggered { x: self.x });
        }
        if self.operation_time == self.get_operation_length() {
            self.finish_operation();
        }
        return breakpoint_triggered;
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut result = 0;
        let initial_breakpoint = 20;
        let breakpoint_increment = 40;
        let last_breakpoint = 220;
        let mut cpu = CPU::new(initial_breakpoint);
        self.input.iter().for_each(|(op, val)| {
            cpu.read_operation(*op, *val);
            while cpu.is_operation_in_progress() {
                if let Some(breakpoint_triggered) = cpu.next_cycle() {
                    result += cpu.breakpoint as isize * breakpoint_triggered.x;
                    if cpu.breakpoint < last_breakpoint {
                        cpu.breakpoint += breakpoint_increment
                    }
                }
            }
        });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut cpu = CPU::new(0);
        self.input.iter().for_each(|(op, val)| {
            cpu.read_operation(*op, *val);
            while cpu.is_operation_in_progress() {
                cpu.next_cycle();
            }
        });
        let result = cpu
            .crt
            .iter()
            .map(|vec| vec.iter().collect::<String>() + "\n")
            .collect::<String>();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "13140";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
