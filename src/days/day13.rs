// abandon all hope ye who enter here

extern crate utils;

use std::cmp::Ordering;

use utils::ChallengeSolver;

pub struct Solver {
    input: Vec<(String, String)>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let mut prev_char = '\0';
        let input = input
            .split(|c| {
                let mut result = false;
                if prev_char == '\n' && c == '\n' {
                    result = true;
                }
                prev_char = c;
                result
            })
            .map(|pair| {
                let (left_packet, right_packet) = pair.split_once('\n').unwrap();
                (String::from(left_packet), String::from(right_packet))
            })
            .collect::<Vec<(String, String)>>();
        Solver { input }
    }

    fn prepare_packet(packet: &str) -> Vec<&str> {
        packet
            .split_inclusive(&[']', '['][..])
            .flat_map(|entry| {
                entry
                    .split(',')
                    .flat_map(|entrier| {
                        // i can't i just can't this is not possible
                        // please revoke my script kiddie license
                        // yes im' goign mad
                        if entrier == "" {
                            vec![]
                        } else if entrier == "]" {
                            vec![entrier]
                        } else if entrier.contains(']') {
                            vec![entrier.split_once(']').unwrap().0, "]"]
                        } else {
                            vec![entrier]
                        }
                    })
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<&str>>()
    }

    fn inject_lists(left_packet: &mut Vec<&str>, right_packet: &mut Vec<&str>) {
        let mut i = 0;
        while i < left_packet.len() && i < right_packet.len() {
            if left_packet[i] != "[" && right_packet[i] == "[" {
                if left_packet[i] == "]" {
                    break;
                }
                left_packet.insert(i, "[");
                left_packet.insert(i + 2, "]");
            } else if left_packet[i] == "[" && right_packet[i] != "[" {
                if right_packet[i] == "]" {
                    break;
                }
                right_packet.insert(i, "[");
                right_packet.insert(i + 2, "]");
            }
            i += 1
        }
    }

    fn is_in_right_order(left_packet: &str, right_packet: &str) -> (bool, Option<usize>) {
        let mut left_packet = Solver::prepare_packet(left_packet);
        let mut right_packet = Solver::prepare_packet(right_packet);
        Solver::inject_lists(&mut left_packet, &mut right_packet);
        if let Some((i, left, right)) = left_packet
            .iter()
            .zip(right_packet.iter())
            .enumerate()
            .find_map(|(i, (left, right))| {
                if left == right {
                    None
                } else {
                    Some((i, *left, *right))
                }
            })
        {
            if right == "]" {
                (false, Some(i))
            } else if left == "]" {
                (true, Some(i))
            } else {
                let left = left.parse::<usize>().unwrap();
                let right = right.parse::<usize>().unwrap();
                if left < right {
                    (true, Some(i))
                } else {
                    (false, Some(i))
                }
            }
        } else {
            (true, None)
        }
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result =
            self.input
                .iter()
                .enumerate()
                .fold(0, |acc, (i, (left_packet, right_packet))| {
                    let (result, _) = Solver::is_in_right_order(left_packet, right_packet);
                    if result {
                        return acc + i + 1;
                    }
                    acc
                });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let (left_packets, right_packets): (Vec<String>, Vec<String>) =
            self.input.iter().cloned().unzip();
        let mut packets = left_packets
            .iter()
            .chain(right_packets.iter())
            .cloned()
            .collect::<Vec<String>>();
        packets.push(String::from("[[2]]"));
        packets.push(String::from("[[6]]"));
        packets.sort_by(|left_packet, right_packet| {
            let (result, _) =
                Solver::is_in_right_order(left_packet.as_str(), right_packet.as_str());
            if result {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        });
        let get_decoder = |decoder: &str| {
            packets
                .iter()
                .enumerate()
                .find(|(_, packet)| *packet == decoder)
                .unwrap()
                .0
                + 1
        };
        let first_decoder_pos = get_decoder("[[2]]");
        let second_decoder_pos = get_decoder("[[6]]");
        let result = first_decoder_pos * second_decoder_pos;
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "13";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example1_only_good_ones() {
        let solver = Solver::new(String::from(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[[4,4],4,4]
[[4,4],4,4,4]

[]
[3]",
        ));
        let result = "10";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn packet_cleaning_test() {
        let result = vec!["[", "10", "13", "[", "3", "11", "]", "42", "[", "]", "]"];

        let answer = Solver::prepare_packet("[10,13,[3,11],42,[]]");

        assert_eq!(&result, &answer);
    }

    #[test]
    fn list_injecting_test_should_be_equal() {
        let mut list1 = vec!["[", "[", "10", "]", "]"];
        let mut list2 = vec!["[", "10", "]"];

        Solver::inject_lists(&mut list1, &mut list2);

        assert_eq!(&list1, &list2);
    }

    #[test]
    fn list_injecting_test_actual_example() {
        let mut list1 = vec!["[", "[", "1", "]", "[", "2", "3", "4", "]", "]"];
        let mut list2 = vec!["[", "[", "1", "]", "4", "]"];

        let expected_list1 = list1.clone();
        Solver::inject_lists(&mut list1, &mut list2);
        let expected_list2 = vec!["[", "[", "1", "]", "[", "4", "]", "]"];

        assert_eq!(&list1, &expected_list1);
        assert_eq!(&list2, &expected_list2)
    }

    #[test]
    fn list_injecting_test_lists_only() {
        let mut list1 = vec!["[", "[", "[", "]", "]", "]"];
        let mut list2 = vec!["[", "[", "]", "]"];

        let expected_list1 = list1.clone();
        let expected_list2 = list2.clone();
        Solver::inject_lists(&mut list1, &mut list2);

        assert_eq!(&list1, &expected_list1);
        assert_eq!(&list2, &expected_list2);
    }

    #[test]
    fn order_test_single_list() {
        let list1 = "[1,1,3,1,1]";
        let list2 = "[1,1,5,1,1]";

        let (result, last_i) = Solver::is_in_right_order(list1, list2);

        assert_eq!(&result, &true);
        assert_eq!(&last_i, &Some(3));
    }

    #[test]
    fn order_test_lists_only() {
        let list1 = "[[[]]]";
        let list2 = "[[]]";

        let (result, last_i) = Solver::is_in_right_order(list1, list2);

        assert_eq!(&result, &false);
        assert_eq!(&last_i, &Some(2));
    }

    #[test]
    fn order_test_added_list() {
        let list1 = "[[1],[2,3,4]]";
        let list2 = "[[1],4]";

        let (result, last_i) = Solver::is_in_right_order(list1, list2);

        assert_eq!(&result, &true);
        assert_eq!(&last_i, &Some(5));
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "140";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }
}
