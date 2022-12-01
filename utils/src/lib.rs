extern crate reqwest;

pub mod day;
pub mod network;

pub use day::Day;

pub fn extract_integers_from_string<T: std::str::FromStr>(input: &String) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_digit(10) && c != '-')
                .filter_map(|s| s.parse().ok())
                .collect()
        }).collect()
}

pub fn extract_unsigned_integers_from_string(input: &String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_digit(10))
                .filter_map(|s| s.parse().ok())
                .collect()
        }).collect()
}

pub fn get_char_arrays(input: &String) -> Vec<Vec<char>> {
        input.lines().filter(|line| !line.is_empty()).map(|line| line.trim().chars().collect::<Vec<char>>()).collect()
}