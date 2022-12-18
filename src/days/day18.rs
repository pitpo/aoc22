// i'm giving up on thinking, don't judge please
// don't read that either :|

extern crate utils;

use utils::ChallengeSolver;

use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    z: usize,
}

impl Node {
    fn is_x_bounded(&self, a: &Node, b: &Node) -> bool {
        if self.y != a.y || self.y != b.y || self.z != a.z || self.z != b.z {
            return false;
        }
        if self.x < a.x && self.x > b.x {
            return true;
        }
        return false;
    }

    fn is_y_bounded(&self, a: &Node, b: &Node) -> bool {
        if self.x != a.x || self.x != b.x || self.z != a.z || self.z != b.z {
            return false;
        }
        if self.y < a.y && self.y > b.y {
            return true;
        }
        return false;
    }

    fn is_z_bounded(&self, a: &Node, b: &Node) -> bool {
        if self.x != a.x || self.x != b.x || self.y != a.y || self.y != b.y {
            return false;
        }
        if self.z < a.z && self.z > b.z {
            return true;
        }
        return false;
    }
}

impl Sub for &Node {
    type Output = usize;

    fn sub(self, other: &Node) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

struct Shape {
    points: Vec<Node>,
}

impl Shape {
    fn get_num_of_exposed_sides(&self) -> usize {
        let all_sides = self.points.len() * 6;
        let covered_sides = self.points.iter().fold(0, |mut acc, point| {
            let mut adjacent_points = self.points.iter().filter_map(|other_point| {
                if point - other_point == 1 {
                    return Some(());
                }
                None
            });
            while let Some(_) = adjacent_points.next() {
                acc += 1;
            }
            acc
        });
        all_sides - covered_sides
    }
}

pub struct Solver {
    input: Vec<Node>,
    min_point: Node,
    max_point: Node,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let input = input
            .lines()
            .map(|line| {
                let nums = line
                    .trim()
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>();
                Node {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                }
            })
            .collect::<Vec<Node>>();
        let find_extreme = |ord: std::cmp::Ordering, init_node: Node| {
            input.iter().fold(init_node, |mut extreme, node| {
                if node.x.cmp(&extreme.x) == ord {
                    extreme.x = node.x;
                }
                if node.y.cmp(&extreme.y) == ord {
                    extreme.y = node.y;
                }
                if node.z.cmp(&extreme.z) == ord {
                    extreme.z = node.z;
                }
                extreme
            })
        };
        let min_point = find_extreme(
            std::cmp::Ordering::Less,
            Node {
                x: usize::MAX,
                y: usize::MAX,
                z: usize::MAX,
            },
        );
        let max_point = find_extreme(std::cmp::Ordering::Greater, Node { x: 0, y: 0, z: 0 });
        Solver {
            input,
            min_point,
            max_point,
        }
    }

    fn attach_cube(shapes: &mut Vec<Shape>, cube: &Node) {
        let shape_to_inject = shapes
            .iter()
            .enumerate()
            .filter_map(|(i, shape)| {
                return shape
                    .points
                    .iter()
                    .filter_map(|point| {
                        if point - cube == 1 {
                            return Some(i);
                        }
                        return None;
                    })
                    .next();
            })
            .next();
        if shape_to_inject.is_some() {
            shapes[shape_to_inject.unwrap()].points.push(*cube);
        } else {
            shapes.push(Shape {
                points: vec![*cube],
            });
        }
    }

    fn merge_shapes(shapes: &mut Vec<Shape>) {
        let mut len_before_merge = 0;
        while len_before_merge != shapes.len() {
            len_before_merge = shapes.len();
            let mut i = 0;
            while i + 1 < shapes.len() {
                let mut j = i + 1;
                while j < shapes.len() {
                    let are_shapes_adjacent = shapes[i]
                        .points
                        .iter()
                        .filter_map(|point| {
                            shapes[j]
                                .points
                                .iter()
                                .filter_map(|other_point| {
                                    if point - other_point == 1 {
                                        return Some(());
                                    }
                                    return None;
                                })
                                .next()
                        })
                        .next()
                        .is_some();
                    if are_shapes_adjacent {
                        let mut other = shapes[j].points.clone();
                        shapes[i].points.append(&mut other);
                        shapes.remove(j);
                    } else {
                        j += 1;
                    }
                }
                i += 1;
            }
        }
    }

    fn fill_gaps(shapes: &mut Vec<Shape>, min_point: &Node, max_point: &Node) {
        // the bruter, the better
        // i swear each day i'm getting more and more ashamed of my "solutions"
        let mut len_before_inject = 0;
        while len_before_inject != shapes.len() {
            len_before_inject = shapes.len();
            for x in min_point.x + 1..max_point.y {
                for y in min_point.y + 1..max_point.y {
                    for z in min_point.z + 1..max_point.z {
                        let node = Node { x, y, z };
                        let mut is_x_bounded = false;
                        let mut is_y_bounded = false;
                        let mut is_z_bounded = false;
                        let mut point_exists = false;
                        shapes.iter().for_each(|shape| {
                            shape.points.iter().for_each(|point| {
                                if node.x == point.x && node.y == point.y && node.z == point.z {
                                    point_exists = true;
                                }
                                if !point_exists {
                                    shapes.iter().for_each(|other_shape| {
                                        other_shape.points.iter().for_each(|other_point| {
                                            if node.is_x_bounded(point, other_point) {
                                                is_x_bounded = true;
                                            }
                                            if node.is_y_bounded(point, other_point) {
                                                is_y_bounded = true;
                                            }
                                            if node.is_z_bounded(point, other_point) {
                                                is_z_bounded = true;
                                            }
                                        })
                                    })
                                }
                            })
                        });
                        if is_x_bounded && is_y_bounded && is_z_bounded && !point_exists {
                            let shape = Shape { points: vec![node] };
                            shapes.push(shape);
                        }
                    }
                }
            }
        }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let mut shapes: Vec<Shape> = Vec::new();
        self.input.iter().for_each(|node| {
            Solver::attach_cube(&mut shapes, node);
        });
        Solver::merge_shapes(&mut shapes);
        let result = shapes
            .iter()
            .fold(0, |acc, shape| acc + shape.get_num_of_exposed_sides());
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut shapes: Vec<Shape> = Vec::new();
        self.input.iter().for_each(|node| {
            Solver::attach_cube(&mut shapes, node);
        });
        Solver::fill_gaps(&mut shapes, &self.min_point, &self.max_point);
        Solver::merge_shapes(&mut shapes);
        let result = shapes
            .iter()
            .fold(0, |acc, shape| acc + shape.get_num_of_exposed_sides());
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5",
        )
    }

    #[test]
    fn surface_calculation_test() {
        let solver = Solver::new(String::from(
            "1,1,1
        2,1,1",
        ));
        let result = "10";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "64";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "58";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
