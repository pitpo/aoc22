extern crate utils;

use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<Blueprint>,
}

#[derive(Debug)]
struct Blueprint {
    orebot_ore_cost: usize,
    claybot_ore_cost: usize,
    obsibot_ore_cost: usize,
    obsibot_clay_cost: usize,
    geobot_ore_cost: usize,
    geobot_obsidian_cost: usize,
    max_ore_cost: usize,
}

#[derive(Debug, Clone, Copy, Hash)]
struct Savepoint {
    time: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    orebots: usize,
    claybots: usize,
    obsibots: usize,
    geobots: usize,
}

impl Savepoint {
    fn generate_resources(&mut self) {
        self.time += 1;
        self.ore += self.orebots;
        self.clay += self.claybots;
        self.obsidian += self.obsibots;
        self.geode += self.geobots;
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let nums = line
                    .trim()
                    .split_whitespace()
                    .filter_map(|word| word.parse::<usize>().ok())
                    .collect::<Vec<usize>>();
                let ore_requirements = vec![nums[0], nums[1], nums[2], nums[4]];
                Blueprint {
                    orebot_ore_cost: nums[0],
                    claybot_ore_cost: nums[1],
                    obsibot_ore_cost: nums[2],
                    obsibot_clay_cost: nums[3],
                    geobot_ore_cost: nums[4],
                    geobot_obsidian_cost: nums[5],
                    max_ore_cost: *ore_requirements.iter().max().unwrap(),
                }
            })
            .collect::<Vec<Blueprint>>();
        Solver { input }
    }

    fn run_simulation(blueprint: &Blueprint, time_limit: usize) -> usize {
        let initial_savepoint = Savepoint {
            time: 0,
            ore: 0,
            orebots: 1,
            clay: 0,
            claybots: 0,
            obsidian: 0,
            obsibots: 0,
            geode: 0,
            geobots: 0,
        };
        let mut visited = HashSet::with_capacity(100000);
        let mut stack = Vec::with_capacity(100000);
        stack.push(initial_savepoint);
        let mut max_geodes = 0;
        while !stack.is_empty() {
            let mut cur_save = stack.pop().unwrap();
            let cur_save_hash = cur_save.get_hash();
            if visited.contains(&cur_save_hash) {
                continue;
            }
            if cur_save.time == time_limit {
                max_geodes = std::cmp::max(max_geodes, cur_save.geode);
                continue;
            }
            visited.insert(cur_save_hash);
            // try building each from most to least complex
            // note to self: you can't build more than one bot in a turn, so it's pointless to overproduce resources
            if cur_save.ore >= blueprint.geobot_ore_cost
                && cur_save.obsidian >= blueprint.geobot_obsidian_cost
            {
                let mut new_save = cur_save.clone();
                new_save.generate_resources();
                new_save.ore -= blueprint.geobot_ore_cost;
                new_save.obsidian -= blueprint.geobot_obsidian_cost;
                new_save.geobots += 1;
                stack.push(new_save);
                continue; // i mean if we can do it, we totally should, as it is the ultimate victory goal, right?
            }
            if cur_save.obsibots < blueprint.geobot_obsidian_cost
                && cur_save.ore >= blueprint.obsibot_ore_cost
                && cur_save.clay >= blueprint.obsibot_clay_cost
            {
                let mut new_save = cur_save.clone();
                new_save.generate_resources();
                new_save.ore -= blueprint.obsibot_ore_cost;
                new_save.clay -= blueprint.obsibot_clay_cost;
                new_save.obsibots += 1;
                stack.push(new_save);
            }
            if cur_save.claybots < blueprint.obsibot_clay_cost
                && cur_save.ore >= blueprint.claybot_ore_cost
            {
                let mut new_save = cur_save.clone();
                new_save.generate_resources();
                new_save.ore -= blueprint.claybot_ore_cost;
                new_save.claybots += 1;
                stack.push(new_save)
            }
            if cur_save.orebots < blueprint.max_ore_cost
                && cur_save.ore >= blueprint.orebot_ore_cost
            {
                let mut new_save = cur_save.clone();
                new_save.generate_resources();
                new_save.ore -= blueprint.orebot_ore_cost;
                new_save.orebots += 1;
                stack.push(new_save);
            }
            cur_save.generate_resources();
            stack.push(cur_save);
        }
        max_geodes
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let time_limit = 24;
        let max_geodes = self
            .input
            .iter()
            .enumerate()
            .map(|(i, blueprint)| Solver::run_simulation(blueprint, time_limit) * (i + 1))
            .collect::<Vec<usize>>();
        let result = max_geodes.iter().sum::<usize>();
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let time_limit = 32;
        let max_geodes = self
            .input
            .iter()
            .take(3)
            .map(|blueprint| Solver::run_simulation(blueprint, time_limit))
            .collect::<Vec<usize>>();
        let result = max_geodes.iter().product::<usize>();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.")
    }

    #[ignore]
    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "33";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[ignore]
    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "3472";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
