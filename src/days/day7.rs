extern crate utils;

use std::collections::HashMap;

use utils::ChallengeSolver;

pub struct Solver {
    directory_files: HashMap<String, Vec<(String, usize)>>,
    directory_children: HashMap<String, Vec<String>>,
}

impl Solver {
    pub fn new(input: String) -> Solver {
        let mut cwd = String::new();
        let mut dir_stack: Vec<String> = vec![];
        let dir_separator = String::from("/");
        let mut directory_files: HashMap<String, Vec<(String, usize)>> = HashMap::new();
        let mut directory_children: HashMap<String, Vec<String>> = HashMap::new();
        directory_files.insert(dir_separator.clone(), vec![]);
        directory_children.insert(dir_separator.clone(), vec![]);
        input.lines().for_each(|line| {
            let line = line.trim();
            if line.starts_with('$') {
                let command = line.split_whitespace().nth(1).unwrap();
                if command.eq("cd") {
                    let dir = line.split_whitespace().nth(2).unwrap();
                    if dir.eq(&dir_separator) {
                        cwd = String::from(dir_separator.clone());
                        dir_stack = vec![];
                    } else if dir.eq("..") {
                        let split_pos = cwd.rfind('/').unwrap();
                        let (result, _) = cwd.split_at(split_pos);
                        cwd = String::from(result);
                        dir_stack.pop();
                    } else {
                        cwd = cwd.clone() + &dir_separator + &String::from(dir);
                        dir_stack.push(cwd.clone());
                    }
                }
            } else {
                let (info, name) = line.split_once(" ").unwrap();
                let (info, name) = (String::from(info), String::from(name));
                if info.eq("dir") {
                    let dir = cwd.clone() + &dir_separator + &String::from(name);
                    directory_files.insert(dir.clone(), vec![]);
                    directory_children.insert(dir.clone(), vec![]);
                    directory_children.get_mut(&cwd).unwrap().push(dir.clone());
                } else {
                    directory_files
                        .get_mut(&cwd)
                        .unwrap()
                        .push((name, info.parse().unwrap()));
                }
            }
        });
        Solver {
            directory_files,
            directory_children,
        }
    }

    fn get_dir_size(&self, name: &String) -> usize {
        let mut total_size = self
            .directory_files
            .get(name)
            .unwrap()
            .iter()
            .fold(0, |acc, (_, size)| acc + size);
        self.directory_children
            .get(name)
            .unwrap()
            .iter()
            .for_each(|child| total_size += self.get_dir_size(child));
        return total_size;
    }

    fn get_required_space(in_use: usize) -> usize {
        let total_available_space = 70000000;
        let total_required_space = 30000000;
        return total_required_space - (total_available_space - in_use);
    }
}

impl ChallengeSolver for Solver {
    fn get_part_a_result(&self) -> String {
        let result = self.directory_files.keys().fold(0, |acc, dir| {
            let dir_size = self.get_dir_size(dir);
            if dir_size <= 100000 {
                return acc + dir_size;
            }
            acc
        });
        String::from(result.to_string())
    }
    fn get_part_b_result(&self) -> String {
        let mut dir_sizes = self
            .directory_children
            .keys()
            .map(|dir| self.get_dir_size(dir))
            .collect::<Vec<usize>>();
        dir_sizes.sort();
        let required_space = Solver::get_required_space(self.get_dir_size(&String::from("/")));
        let result = dir_sizes
            .iter()
            .rev()
            .take_while(|val| val >= &&required_space)
            .last()
            .unwrap();
        String::from(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k",
        )
    }

    #[test]
    fn example1() {
        let solver = Solver::new(get_input());
        let result = "95437";

        let answer = solver.get_part_a_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn example2() {
        let solver = Solver::new(get_input());
        let result = "24933642";

        let answer = solver.get_part_b_result();

        assert_eq!(&result, &answer);
    }

    #[test]
    fn real_math_hours() {
        let result = 8381165;
        let answer = Solver::get_required_space(48381165);
        assert_eq!(&result, &answer);
    }
}
