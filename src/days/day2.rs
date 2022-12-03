extern crate utils;

use utils::Day;

pub struct Day2 {
    input: Vec<(i16, i16)>
}

impl Day2 {
    pub fn new(input: String) -> Day2 {
        let input: Vec<(i16, i16)> = input
            .lines()
            .map(|line| {
                let trimmed_line = line.trim();
                return (
                    (trimmed_line.chars().nth(0).unwrap().clone() as i16) - ('A' as i16) + 1,
                    (trimmed_line.chars().nth_back(0).unwrap().clone() as i16) - ('X' as i16) + 1,
                );
            })
            .collect::<Vec<(i16, i16)>>();
        Day2 { input }
    }
}

impl Day for Day2 {
    fn get_part_a_result(&self) -> String {
        let result = self.input.iter().fold(0, |mut acc, (x, y)| {
            acc += y;
            let diff = (x - y).rem_euclid(3);
            if diff == 0 {
                acc += 3;
            }
            else if diff == 2 {
                acc += 6;
            } 
            return acc;
        });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let result = self.input.iter().fold(0, |mut acc, (x, y)| {
            let mut point_map = [1, 2, 3];
            if *y == 1 {
                point_map.rotate_right(1);
            }
            else if *y == 3 {
                point_map.rotate_left(1);
            }
            acc += point_map[(x - 1) as usize];
            acc += (y - 1) * 3;
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
            "A Y
        B X
        C Z",
        )
    }

    #[test]
    fn example1() {
        let solver = Day2::new(get_input());
        let result = "15";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Day2::new(get_input());
        let result = "12";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn euclidean_modulo_test() {
        let solver = Day2::new(String::from("A Y"));
        let result = "8";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }
}
