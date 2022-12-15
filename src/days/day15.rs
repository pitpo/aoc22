extern crate utils;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
use utils::{plane::Coordinates, ChallengeSolver};

pub struct Solver {
    input: Vec<(Coordinates, Coordinates)>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let numbers = line
                    .trim()
                    .split(&[' ', '=', ',', ':'])
                    .filter_map(|word| word.parse::<isize>().ok())
                    .collect::<Vec<isize>>();
                return (
                    Coordinates::new(numbers[0], numbers[1]),
                    Coordinates::new(numbers[2], numbers[3]),
                );
            })
            .collect::<Vec<(Coordinates, Coordinates)>>();
        Solver { input }
    }

    fn get_ranges_for_each_y(&self) -> HashMap<isize, Vec<(isize, isize)>> {
        let mut ranges = HashMap::new();
        self.input.iter().for_each(|(sensor, beacon)| {
            let distance = (sensor.get_x().abs_diff(beacon.get_x())
                + sensor.get_y().abs_diff(beacon.get_y())) as isize;
            let mut insert_range = |y: isize, length: isize| {
                if !ranges.contains_key(&y) {
                    ranges.insert(y, vec![]);
                }
                ranges
                    .get_mut(&y)
                    .unwrap()
                    .push((sensor.get_x() - length, sensor.get_x() + length + 1));
            };
            for i in 0..distance {
                insert_range(sensor.get_y() - distance + i, i);
            }
            for i in 0..distance + 1 {
                insert_range(sensor.get_y() + i, distance - i);
            }
        });
        ranges.iter_mut().for_each(|mut entry| {
            Solver::merge_ranges(&mut entry.1);
        });
        return ranges;
    }

    fn merge_ranges(ranges: &mut Vec<(isize, isize)>) {
        ranges.sort_by(|a, b| {
            if a == b {
                return Ordering::Equal;
            }
            if a.0 < b.0 || (a.0 == b.0 && a.1 < b.1) {
                return Ordering::Less;
            }
            return Ordering::Greater;
        });
        let mut i = 0;
        while i + 1 < ranges.len() {
            if ranges[i + 1].1 < ranges[i].1 {
                ranges.remove(i + 1);
            } else if ranges[i + 1].0 <= ranges[i].1 {
                ranges[i].1 = ranges[i + 1].1;
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn get_num_of_special_fields(&self) -> HashMap<isize, isize> {
        let mut field_nums = HashMap::new();
        let mut calculated_fields: HashSet<(isize, isize)> = HashSet::new();
        let mut update_y = |coords: &Coordinates| {
            if !calculated_fields.contains(&(coords.get_x(), coords.get_y())) {
                let y = coords.get_y();
                if field_nums.contains_key(&y) {
                    *field_nums.get_mut(&y).unwrap() += 1;
                } else {
                    field_nums.insert(y, 1);
                }
                calculated_fields.insert((coords.get_x(), coords.get_y()));
            }
        };
        self.input.iter().for_each(|(sensor, beacon)| {
            update_y(sensor);
            update_y(beacon);
        });
        return field_nums;
    }

    fn get_num_of_covered_position(
        y: isize,
        ranges: &HashMap<isize, Vec<(isize, isize)>>,
        special_field_num: &HashMap<isize, isize>,
    ) -> isize {
        let zero: isize = 0;
        return ranges
            .get(&y)
            .unwrap()
            .iter()
            .fold(0, |acc, range| acc + range.1 - range.0)
            - special_field_num.get(&y).unwrap_or(&zero);
    }

    fn get_hole(ranges: &HashMap<isize, Vec<(isize, isize)>>) -> Coordinates {
        ranges
            .iter()
            .filter_map(|(y, ranges)| {
                if *y < 0 || *y >= 4000000 || ranges.len() == 1 {
                    return None;
                }
                for i in 0..ranges.len() - 1 {
                    if ranges[i + 1].0 - ranges[i].1 == 1 {
                        return Some(Coordinates::new(ranges[i].1, *y));
                    }
                }
                None
            })
            .next()
            .unwrap()
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let ranges = self.get_ranges_for_each_y();
        let special_field_num = self.get_num_of_special_fields();
        let result = Solver::get_num_of_covered_position(2000000, &ranges, &special_field_num);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let ranges = self.get_ranges_for_each_y();
        let result_coords = Solver::get_hole(&ranges);
        let result = result_coords.get_x() * 4000000 + result_coords.get_y();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let ranges = solver.get_ranges_for_each_y();
        let special_field_num = solver.get_num_of_special_fields();
        let result = 26;

        let answer = Solver::get_num_of_covered_position(10, &ranges, &special_field_num);

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example1_with_a_hole() {
        let solver = Solver::new(get_input());
        let ranges = solver.get_ranges_for_each_y();
        let special_field_num = solver.get_num_of_special_fields();
        let result = 27;

        let answer = Solver::get_num_of_covered_position(11, &ranges, &special_field_num);

        assert_eq!(&result, &answer);
    }

    #[test]
    fn ranges_test() {
        let mut ranges = vec![(11, 12), (0, 7), (-3, 0), (7, 8), (2, 5), (4, 10), (11, 13)];
        let expected_ranges = vec![(-3, 10), (11, 13)];

        Solver::merge_ranges(&mut ranges);

        assert_eq!(&expected_ranges, &ranges);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let ranges = solver.get_ranges_for_each_y();
        let result = Coordinates::new(14, 11);

        let answer = Solver::get_hole(&ranges);

        assert_eq!(&result.get_x(), &answer.get_x());
        assert_eq!(&result.get_y(), &answer.get_y());
    }
}
