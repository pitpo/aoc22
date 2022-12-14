extern crate utils;

use std::{collections::HashSet, mem::swap};

use utils::{
    plane::{Boundary, Direction},
    ChallengeSolver,
};

pub struct Solver {
    rocks: HashSet<(isize, isize)>,
    boundary: Boundary,
    max_height: isize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let mut max_height = 0;
        let input = input
            .lines()
            .flat_map(|line| {
                let coords = line.trim().split("->").collect::<Vec<&str>>();
                let mut i = 0;
                let mut rocks: HashSet<(isize, isize)> = HashSet::new();
                while i < coords.len() - 1 {
                    let extract_coords = |coords_str: &str| {
                        let (x, y) = coords_str.split_once(',').unwrap();
                        return (
                            x.trim().parse::<isize>().unwrap(),
                            y.trim().parse::<isize>().unwrap(),
                        );
                    };
                    let (mut left_x, mut left_y) = extract_coords(coords[i]);
                    let (mut right_x, mut right_y) = extract_coords(coords[i + 1]);
                    if left_x != right_x && right_x < left_x {
                        swap(&mut left_x, &mut right_x);
                    }
                    if left_y != right_y && right_y < left_y {
                        swap(&mut right_y, &mut left_y);
                    }
                    if right_y > max_height {
                        max_height = right_y;
                    }
                    if left_x < right_x {
                        for i in left_x..right_x + 1 {
                            rocks.insert((left_y, i));
                        }
                    }
                    if left_y < right_y {
                        for i in left_y..right_y + 1 {
                            rocks.insert((i, left_x));
                        }
                    }
                    i += 1;
                }
                rocks
            })
            .collect::<HashSet<(isize, isize)>>();
        let boundary = Boundary::new_infinite_boundary();
        Solver {
            rocks: input,
            boundary,
            max_height,
        }
    }

    fn try_moving_sand(
        &self,
        fallen_sand: &HashSet<(isize, isize)>,
        cur_sand_pos: (isize, isize),
    ) -> Option<(isize, isize)> {
        let direction_order = [Direction::None, Direction::Left, Direction::Right];
        let result = direction_order
            .iter()
            .filter_map(|dir| {
                let new_pos = self
                    .boundary
                    .move_iterator(cur_sand_pos, &Direction::Down)
                    .unwrap();
                let new_pos = self.boundary.move_iterator(new_pos, dir).unwrap();
                if self.rocks.contains(&new_pos)
                    || fallen_sand.contains(&new_pos)
                    || new_pos.0 == self.max_height + 2
                {
                    return None;
                } else {
                    return Some(new_pos);
                }
            })
            .next();
        result
    }

    fn add_sand(&self, fallen_sand: &mut HashSet<(isize, isize)>) -> (isize, isize) {
        let mut sand_pos = (0, 500);
        while let Some(new_sand_pos) = self.try_moving_sand(&fallen_sand, sand_pos) {
            sand_pos = new_sand_pos;
        }
        fallen_sand.insert(sand_pos);
        return sand_pos;
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut fallen_sand: HashSet<(isize, isize)> = HashSet::new();
        while self.add_sand(&mut fallen_sand).0 != self.max_height + 1 {}
        let result = fallen_sand.len() - 1;
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut fallen_sand: HashSet<(isize, isize)> = HashSet::new();
        while self.add_sand(&mut fallen_sand) != (0, 500) {}
        let result = fallen_sand.len();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "24";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn parsing_test() {
        let solver = Solver::new(get_input());
        let result = solver.rocks.len();

        let answer = 20;

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "93";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
