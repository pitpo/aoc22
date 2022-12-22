extern crate utils;

use utils::{
    plane::{Boundary, Direction},
    ChallengeSolver,
};

pub struct Solver {
    map: Vec<Vec<Tile>>,
    instructions: Vec<Instruction>,
    starting_point: (usize, usize),
    boundary: Boundary,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Void,
    Floor,
    Wall,
}

#[derive(Debug)]
struct Instruction {
    distance_to_go: usize,
    turn: Direction,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let height = map.lines().count();
        let width = map.lines().max_by_key(|line| line.len()).unwrap().len();
        let boundary = Boundary::new_array_boundary(width, height);
        let map = map
            .lines()
            .map(|line| {
                let mut row = Vec::with_capacity(width);
                line.chars().for_each(|c| {
                    if c == ' ' {
                        row.push(Tile::Void);
                    } else if c == '.' {
                        row.push(Tile::Floor);
                    } else if c == '#' {
                        row.push(Tile::Wall);
                    }
                });
                for i in row.len()..width {
                    row.push(Tile::Void);
                }
                row
            })
            .collect::<Vec<Vec<Tile>>>();
        let starting_x = map[0]
            .iter()
            .enumerate()
            .find_map(|(i, tile)| {
                if tile == &Tile::Floor {
                    return Some(i);
                }
                None
            })
            .unwrap();
        let instructions = instructions
            .trim()
            .split_inclusive(&['R', 'L'])
            .map(|instruction| {
                println!("{instruction}");
                if instruction.contains(&['R', 'L']) {
                    let (num, dir) = instruction.split_at(instruction.len() - 1);
                    let dir = match dir {
                        "R" => Direction::Right,
                        "L" => Direction::Left,
                        _ => {
                            panic!("just how in the world")
                        }
                    };
                    return Instruction {
                        distance_to_go: num.parse().unwrap(),
                        turn: dir,
                    };
                } else {
                    return Instruction {
                        distance_to_go: instruction.parse().unwrap(),
                        turn: Direction::None,
                    };
                }
            })
            .collect::<Vec<Instruction>>();
        Solver {
            map,
            instructions,
            starting_point: (0, starting_x),
            boundary,
        }
    }

    fn run_simulation(&self) -> (usize, (usize, usize)) {
        let mut cur_pos = self.starting_point;
        let mut cur_dir = Direction::Right;
        self.instructions.iter().for_each(|inst| {
            let mut can_move = true;
            let mut distance_made = 0;
            while can_move && distance_made < inst.distance_to_go {
                let mut wrap_around = false;
                if let Some(new_pos) = self.boundary.move_array_iterator(cur_pos, &cur_dir) {
                    if self.map[new_pos.0][new_pos.1] == Tile::Floor {
                        distance_made += 1;
                        cur_pos = new_pos;
                    } else if self.map[new_pos.0][new_pos.1] == Tile::Void {
                        wrap_around = true;
                    } else {
                        can_move = false;
                    }
                } else {
                    wrap_around = true;
                }

                if wrap_around {
                    let mut new_pos = self.boundary.wrap_array_iterator(cur_pos, &cur_dir);
                    while self.map[new_pos.0][new_pos.1] == Tile::Void {
                        new_pos = self
                            .boundary
                            .move_array_iterator(new_pos, &cur_dir)
                            .unwrap();
                    }
                    if self.map[new_pos.0][new_pos.1] == Tile::Floor {
                        distance_made += 1;
                        cur_pos = new_pos;
                    } else {
                        can_move = false;
                    }
                }
            }
            cur_dir = cur_dir.rotate(&inst.turn);
        });
        let facing_value = match cur_dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
            _ => {
                panic!("this is impossible")
            }
        };
        (facing_value, cur_pos)
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let (facing, final_point) = self.run_simulation();
        let result = 1000 * (final_point.0 + 1) + 4 * (final_point.1 + 1) + facing;
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = "IMPLEMENT ME";
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "6032";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
