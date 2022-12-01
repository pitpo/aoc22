extern crate utils;

use utils::Day;

pub struct Day1 {
    input: Vec<Vec<u32>>,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        let input = input.lines().fold(vec![vec![]], |mut acc, line| {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                acc.push(vec![]);
            } else {
                acc.last_mut()
                    .unwrap()
                    .push(trimmed_line.parse::<u32>().unwrap());
            }
            acc
        });

        Day1 { input }
    }

    fn get_invertory_list_in_descending_content_size_order(&self) -> Vec<Vec<u32>> {
        let mut inventory = self.input.clone();
        inventory.sort_by_key(|x| x.iter().sum::<u32>());
        inventory.reverse();
        return inventory;
    }
}

impl Day for Day1 {
    fn get_part_a_result(&self) -> String {
        let result: u32 = self
            .get_invertory_list_in_descending_content_size_order()
            .first()
            .unwrap()
            .iter()
            .sum();
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result: u32 = self.get_invertory_list_in_descending_content_size_order()[..3]
            .iter()
            .fold(0, |acc, x| acc + x.iter().sum::<u32>());

        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000",
        )
    }

    #[test]
    fn example1() {
        let solver = Day1::new(get_input());
        let result = "24000";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Day1::new(get_input());
        let result = "45000";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
