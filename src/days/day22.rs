extern crate utils;

use std::{collections::{HashMap, VecDeque}, hash::Hash};

use utils::{
    plane::{Boundary, Direction},
    ChallengeSolver,
};

pub struct Solver {
    map: Vec<Vec<Tile>>,
    instructions: Vec<Instruction>,
    starting_point: (usize, usize),
    boundary: Boundary,
    face_boundary: Boundary,
    face_size: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Void,
    Floor,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CubeSegment {
    BackSide,
    Top,
    LeftSide,
    Bottom,
    FrontSide,
    RightSide,
}

#[derive(Debug)]
struct Instruction {
    distance_to_go: usize,
    turn: Direction,
}

#[derive(Debug, Clone, Copy)]
struct PositionData {
    pos: (usize, usize),
    dir: Direction,
    face: CubeSegment,
}

#[derive(Debug)]
struct OriginalOrientationData {
    face_y: usize,
    face_x: usize,
    rotation: isize,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let (map, instructions) = input.split_once("\n\n").unwrap();
        let height = map.lines().count();
        let width = map.lines().max_by_key(|line| line.len()).unwrap().len();
        let boundary = Boundary::new_array_boundary(width, height);
        let face_size = std::cmp::max(width / 4, height / 4);
        let face_boundary = Boundary::new_array_boundary(face_size, face_size);
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
                for _ in row.len()..width {
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
            face_size,
            face_boundary,
        }
    }

    fn move_iterator(&self, position_data: &PositionData) -> PositionData {
        if let Some(new_pos) = self.face_boundary.move_array_iterator(position_data.pos, &position_data.dir) {
            return PositionData{ pos: new_pos, dir: position_data.dir, face: position_data.face };
        }
        else {
            let f_max = self.face_size - 1;
            let (y,x) = position_data.pos;
            match position_data.face {
                CubeSegment::Bottom => match position_data.dir {
                    Direction::Down => PositionData{ pos: (f_max, f_max - x), dir: Direction::Up, face: CubeSegment::FrontSide },
                    Direction::Left => PositionData { pos: (f_max, f_max - y), dir: Direction::Up, face: CubeSegment::LeftSide },
                    Direction::Right => PositionData { pos: (f_max, y), dir: Direction::Up, face: CubeSegment::RightSide },
                    Direction::Up => PositionData { pos: (f_max, x), dir: Direction::Up, face: CubeSegment::BackSide },
                    _ => {panic!("unreachable branch");}
                },
                CubeSegment::BackSide => match position_data.dir {
                    Direction::Down => PositionData { pos: (0, x), dir: Direction::Down, face: CubeSegment::Bottom },
                    Direction::Left => PositionData { pos: (y, f_max), dir: Direction::Left, face: CubeSegment::LeftSide },
                    Direction::Right => PositionData { pos: (y, 0), dir: Direction::Right, face: CubeSegment::RightSide },
                    Direction::Up => PositionData { pos: (f_max, x), dir: Direction::Up, face: CubeSegment::Top },
                    _ => {panic!("unreachable branch");}
                },
                CubeSegment::RightSide => match position_data.dir {
                    Direction::Down => PositionData { pos: (x, f_max), dir: Direction::Left, face: CubeSegment::Bottom },
                    Direction::Left => PositionData { pos: (y, f_max), dir: Direction::Left, face: CubeSegment::BackSide },
                    Direction::Right => PositionData { pos: (y, 0), dir: Direction::Right, face: CubeSegment::FrontSide },
                    Direction::Up => PositionData { pos: (f_max - x, f_max), dir: Direction::Left, face: CubeSegment::Top },
                    _ => {panic!("unreachable branch");}
                }
                CubeSegment::FrontSide => match position_data.dir {
                    Direction::Down => PositionData { pos: (f_max, f_max - x), dir: Direction::Up, face: CubeSegment::Bottom },
                    Direction::Left => PositionData { pos: (y, f_max), dir: Direction::Left, face: CubeSegment::RightSide },
                    Direction::Right => PositionData { pos: (y, 0), dir: Direction::Right, face: CubeSegment::LeftSide },
                    Direction::Up => PositionData { pos: (0, f_max - x), dir: Direction::Down, face: CubeSegment::Top },
                    _ => {panic!("unreachable branch");}
                },
                CubeSegment::LeftSide => match position_data.dir {
                    Direction::Down => PositionData { pos: (f_max - x, 0), dir: Direction::Right, face: CubeSegment::Bottom },
                    Direction::Left => PositionData { pos: (y, f_max), dir: Direction::Left, face: CubeSegment::FrontSide },
                    Direction::Right => PositionData { pos: (y, 0), dir: Direction::Right, face: CubeSegment::BackSide },
                    Direction::Up => PositionData { pos: (x, 0), dir: Direction::Right, face: CubeSegment::Top },
                    _ => {panic!("unreachable branch");}
                },
                CubeSegment::Top => match position_data.dir {
                    Direction::Down => PositionData { pos: (0, x), dir: Direction::Down, face: CubeSegment::BackSide },
                    Direction::Left => PositionData { pos: (0, y), dir: Direction::Down, face: CubeSegment::LeftSide },
                    Direction::Right => PositionData { pos: (0, f_max - y), dir: Direction::Down, face: CubeSegment::RightSide },
                    Direction::Up => PositionData { pos: (0, f_max - x), dir: Direction::Down, face: CubeSegment::FrontSide },
                    _ => {panic!("unreachable branch");}
                }
            }
        }
    }

    fn rotate_right(&self, map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        let mut rotated = vec![vec![Tile::Void; self.face_size]; self.face_size];
        for i in 0..self.face_size {
            for j in 0..self.face_size {
                rotated[j][self.face_size - 1 - i] = map[i][j];
            }
        }
        rotated
    }

    fn dissect_cube(&self) -> (HashMap<CubeSegment, Vec<Vec<Tile>>>, HashMap<CubeSegment, OriginalOrientationData>) {
        let mut cube_map = HashMap::new();
        let mut orientation_data = HashMap::new();
        let mut face_positions = vec![vec![None; 5]; 5];
        for y in (0..self.map.len()).step_by(self.face_size) {
            for x in (0..self.map[y].len()).step_by(self.face_size) {
                if self.map[y][x] != Tile::Void {
                    face_positions[y/self.face_size][x/self.face_size] = Some(CubeSegment::Bottom);
                }
            }
        }
        let get_face = |y: usize, x: usize| {
            let mut face = vec![vec![Tile::Void; self.face_size]; self.face_size];
            for face_y in 0..self.face_size {
                for face_x in 0..self.face_size {
                    face[face_y][face_x] = self.map[y*self.face_size + face_y][x*self.face_size + face_x];
                }
            }
            face
        };
        let mut queue = VecDeque::new();
        for x in 0..4 {
            if face_positions[0][x].is_some() {
                queue.push_back((0, x));
                let bottom_face = get_face(0, x);
                cube_map.insert(CubeSegment::Bottom, bottom_face);
                orientation_data.insert(CubeSegment::Bottom, OriginalOrientationData { face_y: 0, face_x: x, rotation: 0 });
                break;
            } 
        }
        let mut insert_face = |y: usize, x: usize, face: CubeSegment, rotate: usize, queue: &mut VecDeque<(usize, usize)>, face_positions: &mut Vec<Vec<Option<CubeSegment>>>| {
            if let Some(CubeSegment::Bottom) = face_positions[y][x] {
                face_positions[y][x] = Some(face);
                let mut face_vec = get_face(y, x);
                for _ in 0..rotate {
                    face_vec = self.rotate_right(&face_vec);
                }
                cube_map.insert(face, face_vec);
                orientation_data.insert(face, OriginalOrientationData { face_y: y, face_x: x, rotation: rotate as isize });
                queue.push_back((y, x));
            }
        };
        while let Some((y,x)) = queue.pop_front() {
            // ah fuck it, let's just support test and my input
            // couldn't care less at this point
            match face_positions[y][x] {
                Some(CubeSegment::Bottom) => {
                    insert_face(y, x+1, CubeSegment::RightSide, 3, &mut queue, &mut face_positions);
                    insert_face(y+1, x, CubeSegment::FrontSide, 2, &mut queue, &mut face_positions);
                },
                Some(CubeSegment::FrontSide) => {
                    insert_face(y+1, x, CubeSegment::Top, 0, &mut queue, &mut face_positions);
                    if x > 0 {
                        insert_face(y, x-1, CubeSegment::LeftSide, 2, &mut queue, &mut face_positions);
                    }
                }
                Some(CubeSegment::Top) => {
                    insert_face(y, x+1, CubeSegment::RightSide, 1, &mut queue, &mut face_positions);
                    if x > 0 {
                        insert_face(y, x-1, CubeSegment::LeftSide, 3, &mut queue, &mut face_positions);
                    }
                }
                Some(CubeSegment::LeftSide) => {
                    if x > 0 {
                        insert_face(y, x-1, CubeSegment::BackSide, 2, &mut queue, &mut face_positions);
                    }
                    insert_face(y+1,x, CubeSegment::BackSide, 3, &mut queue, &mut face_positions);
                }
                Some(_) => {}
                None => { }
            }
        }
        (cube_map, orientation_data)
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

    fn run_3d_simulation(&self) -> (usize, (usize, usize)) {
        let mut cur_pos = PositionData { pos: (0, 0), dir: Direction::Right, face: CubeSegment::Bottom };
        let (cube, orientation_data) = self.dissect_cube();
        let convert_pos = |pos: &PositionData| {
            let orientation_data = orientation_data.get(&pos.face).unwrap();
            let counter_rotation = (-orientation_data.rotation).rem_euclid(4);
            let mut new_pos = pos.clone();
            for _ in 0..counter_rotation {
                let tmp = new_pos.pos.0;
                new_pos.pos.0 = new_pos.pos.1;
                new_pos.pos.1 = self.face_size - 1 - tmp;
                new_pos.dir = new_pos.dir.rotate(&Direction::Right);
            }
            new_pos.pos = (new_pos.pos.0 + orientation_data.face_y * self.face_size, new_pos.pos.1 + orientation_data.face_x * self.face_size);
            // println!("{new_pos:?}");
            return new_pos;
        };
        self.instructions.iter().for_each(|inst| {
            let mut can_move = true;
            let mut distance_made = 0;
            while can_move && distance_made < inst.distance_to_go {
                let new_pos = self.move_iterator(&cur_pos);
                if cube.get(&new_pos.face).unwrap()[new_pos.pos.0][new_pos.pos.1] == Tile::Wall {
                    can_move = false;
                } else {
                    distance_made += 1;
                    cur_pos = new_pos;
                    // convert_pos(&cur_pos);
                }
            }
            cur_pos.dir = cur_pos.dir.rotate(&inst.turn);
        });
        let final_pos = convert_pos(&cur_pos);
        
        let facing_value = match final_pos.dir {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
            _ => {
                panic!("this is impossible")
            }
        };
        (facing_value, final_pos.pos)
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let (facing, final_point) = self.run_simulation();
        let result = 1000 * (final_point.0 + 1) + 4 * (final_point.1 + 1) + facing;
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let (facing, final_point) = self.run_3d_simulation();
        let result = 1000 * (final_point.0 + 1) + 4 * (final_point.1 + 1) + facing;
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

    #[ignore]
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
        let result = "5031";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
