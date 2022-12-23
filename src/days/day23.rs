extern crate utils;

use std::collections::{HashMap, HashSet};

use utils::{plane::Direction, ChallengeSolver};

pub struct Solver {
    elves: HashSet<(isize, isize)>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == '#' {
                            return Some((y as isize, x as isize));
                        }
                        None
                    })
                    .collect::<HashSet<(isize, isize)>>()
            })
            .collect::<HashSet<(isize, isize)>>();
        Solver { elves }
    }

    fn get_north_coords((y, x): &(isize, isize)) -> HashSet<(isize, isize)> {
        HashSet::from_iter(
            vec![(y - 1, x - 1), (y - 1, *x), (y - 1, x + 1)]
                .iter()
                .cloned(),
        )
    }

    fn get_south_coords((y, x): &(isize, isize)) -> HashSet<(isize, isize)> {
        HashSet::from_iter(
            vec![(y + 1, x - 1), (y + 1, *x), (y + 1, x + 1)]
                .iter()
                .cloned(),
        )
    }

    fn get_west_coords((y, x): &(isize, isize)) -> HashSet<(isize, isize)> {
        HashSet::from_iter(
            vec![(y - 1, x - 1), (*y, x - 1), (y + 1, x - 1)]
                .iter()
                .cloned(),
        )
    }

    fn get_east_coords((y, x): &(isize, isize)) -> HashSet<(isize, isize)> {
        HashSet::from_iter(
            vec![(y - 1, x + 1), (*y, x + 1), (y + 1, x + 1)]
                .iter()
                .cloned(),
        )
    }

    fn get_surrounding_coords(coords: &(isize, isize)) -> HashSet<(isize, isize)> {
        let mut set = Solver::get_north_coords(coords);
        set.extend(&Solver::get_south_coords(coords));
        set.extend(&Solver::get_west_coords(coords));
        set.extend(Solver::get_east_coords(coords));
        set
    }

    fn cycle_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Up,
            _ => {
                panic!("invalid state");
            }
        }
    }

    fn is_any_elf_in_range(
        elves: &HashSet<(isize, isize)>,
        range: &HashSet<(isize, isize)>,
    ) -> bool {
        elves.intersection(range).count() != 0
    }

    fn try_inserting_move(
        elves: &HashSet<(isize, isize)>,
        moves: &mut HashMap<(isize, isize), Vec<(isize, isize)>>,
        old_pos: &(isize, isize),
        check_direction: Direction,
    ) -> bool {
        let range;
        let new_pos;
        if check_direction == Direction::Up {
            range = Solver::get_north_coords(old_pos);
            new_pos = (old_pos.0 - 1, old_pos.1);
        } else if check_direction == Direction::Down {
            range = Solver::get_south_coords(old_pos);
            new_pos = (old_pos.0 + 1, old_pos.1);
        } else if check_direction == Direction::Left {
            range = Solver::get_west_coords(old_pos);
            new_pos = (old_pos.0, old_pos.1 - 1);
        } else {
            range = Solver::get_east_coords(old_pos);
            new_pos = (old_pos.0, old_pos.1 + 1);
        }
        let mut moved = false;
        if !Solver::is_any_elf_in_range(elves, &range) {
            moves
                .entry(new_pos)
                .and_modify(|v| v.push(*old_pos))
                .or_insert(vec![*old_pos]);
            moved = true;
        }
        moved
    }

    fn run_simulation(&self, is_part_a: bool) -> (HashSet<(isize, isize)>, usize) {
        let mut elves = self.elves.clone();
        let iters = 10;
        let mut current_iter = 0;
        let mut cur_dir = Direction::Up;
        let mut elves_moved = 1;
        while (!is_part_a && elves_moved > 0) || (is_part_a && current_iter < iters) {
            let mut moves: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
            elves.iter().for_each(|elf_coords| {
                let elf_range = Solver::get_surrounding_coords(elf_coords);
                if Solver::is_any_elf_in_range(&elves, &elf_range) {
                    let mut next_dir = cur_dir.clone();
                    let mut moved = false;
                    for _ in 0..4 {
                        if !moved {
                            moved = Solver::try_inserting_move(
                                &elves, &mut moves, elf_coords, next_dir,
                            );
                            next_dir = Solver::cycle_direction(next_dir);
                        }
                    }
                }
            });
            moves.iter().for_each(|(_, current)| {
                if current.len() == 1 {
                    elves.remove(current.first().unwrap());
                }
            });
            moves.iter().for_each(|(destination, current)| {
                if current.len() == 1 {
                    elves.insert(*destination);
                }
            });
            elves_moved = moves.len();
            cur_dir = Solver::cycle_direction(cur_dir);
            current_iter += 1
        }
        (elves, current_iter)
    }

    fn get_boundary_len(elves: &HashSet<(isize, isize)>) -> (isize, isize) {
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);
        elves.iter().for_each(|elf_coords| {
            if elf_coords.0 < min_y {
                min_y = elf_coords.0;
            }
            if elf_coords.0 > max_y {
                max_y = elf_coords.0;
            }
            if elf_coords.1 < min_x {
                min_x = elf_coords.1;
            }
            if elf_coords.1 > max_x {
                max_x = elf_coords.1;
            }
        });
        (max_y - min_y + 1, max_x - min_x + 1)
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let (elves, _) = self.run_simulation(true);
        let boundary_len = Solver::get_boundary_len(&elves);
        let result = boundary_len.0 * boundary_len.1 - (elves.len() as isize);
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let (_, result) = self.run_simulation(false);
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        )
    }

    #[test]
    fn smaller_input() {
        let solver = Solver::new(String::from(
            ".....
        ..##.
        ..#..
        .....
        ..##.
        .....",
        ));
        let result = "25";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "110";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "20";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
