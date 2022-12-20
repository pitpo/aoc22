extern crate utils;

use std::collections::LinkedList;

use utils::ChallengeSolver;

pub struct Solver {
    input: LinkedList<(usize, isize)>
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input.lines().enumerate().map(|(i, line)| {
            (i, line.trim().parse::<isize>().unwrap())
        }).collect::<LinkedList<(usize, isize)>>();
        Solver { input }
    }

    fn get_split_point(original_index: usize, list: &LinkedList<(usize, isize)>) -> (usize, (usize, isize)) {
        list.iter().enumerate().find_map(|(i, item)| {
            if item.0 == original_index {
                return Some((i, *item));
            }
            None
        }).unwrap()
    }

    fn get_mixed_list(&self, current_list: &LinkedList<(usize, isize)>) -> LinkedList<(usize, isize)> {
        let mut list = current_list.clone();
        for i in 0..self.input.len() {
            let (split_point, item) = Solver::get_split_point(i, &list);

            let shift = item.1.rem_euclid(self.input.len() as isize - 1) as usize;
            let mut append_point = (split_point + shift).rem_euclid(self.input.len());
            
            let mut remainder = list.split_off(split_point);
            remainder.pop_front();
            list.append(&mut remainder);

            if split_point + shift == 0 {
                list.push_back(item);
            }
            else 
            {
                // dear diary, today   \/   comprasion was a dick (i was checking '>' instead of '>=')
                if split_point + shift >= self.input.len() {
                    append_point += 1;
                } 
                let mut remainder = list.split_off(append_point);
                remainder.push_front(item);
                list.append(&mut remainder);
            }
        }
        list
    }

    fn get_result(list: &LinkedList<(usize, isize)>) -> isize {
        let zero_pos = list.iter().enumerate().find_map(|(i, item)| {
            if item.1 == 0 {
                return Some(i)
            }
            None
        }).unwrap();
        let first_idx = (zero_pos + 1000) % list.len();
        let second_idx = (zero_pos + 2000) % list.len();
        let third_idx = (zero_pos + 3000) % list.len();
        return list.iter().enumerate().fold(0, |acc, (i, item)| {
            if i == first_idx || i == second_idx || i == third_idx {
                return acc + item.1;
            }
            return acc;
        });
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mixed_list = self.get_mixed_list(&self.input);
        let result = Solver::get_result(&mixed_list);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut mixed_list = self.input.iter().map(|(i, item)| (*i, *item * 811589153)).collect::<LinkedList<(usize, isize)>>();
        for _ in 0..10 {
            mixed_list = self.get_mixed_list(&mixed_list);
        }
        let result = Solver::get_result(&mixed_list);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from("1
        2
        -3
        3
        -2
        0
        4")
    }

    #[test]
    fn list_mix_test() {
        let solver = Solver::new(get_input());
        let result_vec = vec![(0, 1), (1, 2), (2, -3), (6, 4), (5, 0), (3, 3), (4, -2)];
        let result = LinkedList::from_iter(result_vec.iter().cloned());

        let answer = solver.get_mixed_list(&solver.input);

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "3";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "1623178306";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn test_wraping_to_0th_index() {
        let solver = Solver::new(String::from("2
        -2
        0
        -1
        -4
        -1"));
        let result = "0";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }
}