extern crate utils;

use std::collections::HashMap;
use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<String>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| String::from(line.trim()))
            .collect();
        Solver { input: input }
    }

    // using maps instead of set, because expected part 2 to focus on items with multiple occurences
    fn bucket_chars(compartment: &str) -> HashMap<char, usize> {
        let mut buckets: HashMap<char, usize> = HashMap::new();
        compartment
            .chars()
            .for_each(|c| *buckets.entry(c).or_insert(0) += 1);
        return buckets;
    }

    fn transform_char_to_expected_value(c: char) -> usize {
        if c.is_lowercase() {
            return 1 + c as usize - 'a' as usize;
        } else if c.is_uppercase() {
            return 27 + c as usize - 'A' as usize;
        }
        return 0;
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.input.iter().fold(0, |mut acc, rucksack| {
            let (first_comparment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
            let first_comp_buckets = Solver::bucket_chars(first_comparment);
            let second_comp_buckets = Solver::bucket_chars(second_compartment);
            let common_key = first_comp_buckets
                .keys()
                .find(|key| second_comp_buckets.get(key).is_some())
                .unwrap();
            acc += Solver::transform_char_to_expected_value(*common_key);
            return acc;
        });
        String::from(result.to_string())
    }

    fn get_part_b_result(&self) -> String {
        let mut counter = 0;
        let groups = self.input.split_inclusive(|_| {
            counter += 1;
            if counter % 3 == 0 {
                return true;
            } else {
                return false;
            }
        });
        let result = groups.fold(0, |mut acc, chunks| {
            let first_rucksack_buckets = Solver::bucket_chars(chunks[0].as_str());
            let second_rucksack_buckets = Solver::bucket_chars(chunks[1].as_str());
            let third_rucksack_buckets = Solver::bucket_chars(chunks[2].as_str());
            let common_key = first_rucksack_buckets
                .keys()
                .find(|first_key| {
                    second_rucksack_buckets
                        .get_key_value(first_key)
                        .and_then(|(second_key, _)| third_rucksack_buckets.get(second_key))
                        .is_some()
                })
                .unwrap();
            acc += Solver::transform_char_to_expected_value(*common_key);
            return acc;
        });
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "157";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "70";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
