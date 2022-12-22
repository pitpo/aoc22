extern crate utils;

use std::env;
use std::time::SystemTime;
use utils::network::*;
use utils::ChallengeSolver;

mod days;
use days::*;

fn get_solver(day: &str) -> Box<dyn ChallengeSolver> {
    match day {
        "day1" => Box::new(day1::Solver::new(get_input_for_day(2022, 1))),
        "day2" => Box::new(day2::Solver::new(get_input_for_day(2022, 2))),
        "day3" => Box::new(day3::Solver::new(get_input_for_day(2022, 3))),
        "day4" => Box::new(day4::Solver::new(get_input_for_day(2022, 4))),
        "day5" => Box::new(day5::Solver::new(get_input_for_day(2022, 5))),
        "day6" => Box::new(day6::Solver::new(get_input_for_day(2022, 6))),
        "day7" => Box::new(day7::Solver::new(get_input_for_day(2022, 7))),
        "day8" => Box::new(day8::Solver::new(get_input_for_day(2022, 8))),
        "day9" => Box::new(day9::Solver::new(get_input_for_day(2022, 9))),
        "day10" => Box::new(day10::Solver::new(get_input_for_day(2022, 10))),
        "day11" => Box::new(day11::Solver::new(get_input_for_day(2022, 11))),
        "day12" => Box::new(day12::Solver::new(get_input_for_day(2022, 12))),
        "day13" => Box::new(day13::Solver::new(get_input_for_day(2022, 13))),
        "day14" => Box::new(day14::Solver::new(get_input_for_day(2022, 14))),
        "day15" => Box::new(day15::Solver::new(get_input_for_day(2022, 15))),
        "day16" => Box::new(day16::Solver::new(get_input_for_day(2022, 16))),
        "day17" => Box::new(day17::Solver::new(get_input_for_day(2022, 17))),
        "day18" => Box::new(day18::Solver::new(get_input_for_day(2022, 18))),
        "day19" => Box::new(day19::Solver::new(get_input_for_day(2022, 19))),
        "day20" => Box::new(day20::Solver::new(get_input_for_day(2022, 20))),
        "day21" => Box::new(day21::Solver::new(get_input_for_day(2022, 21))),
        "day22" => Box::new(day22::Solver::new(get_input_for_day(2022, 22))),
        // "day23" => Box::new(day23::Solver::new(get_input_for_day(2022, 23))),
        // "day24" => Box::new(day24::Solver::new(get_input_for_day(2022, 24))),
        // "day25" => Box::new(day25::Solver::new(get_input_for_day(2022, 25))),
        _ => panic!("Unknown or missing argument")
    }
}

fn main() {
    let env_arg = env::args().nth(1).unwrap_or_default();
    let solver = get_solver(env_arg.as_ref());

    println!("-----------PART A SOLUTION-----------");
    let timer = SystemTime::now();
    println!("{}", solver.get_part_a_result());
    let duration = SystemTime::now().duration_since(timer).unwrap();
    println!(
        "Took {}.{:09}s",
        duration.as_secs(),
        duration.subsec_nanos()
    );
    println!("-----------PART B SOLUTION-----------");
    let timer = SystemTime::now();
    println!("{}", solver.get_part_b_result());
    let duration = SystemTime::now().duration_since(timer).unwrap();
    println!(
        "Took {}.{:09}s",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
