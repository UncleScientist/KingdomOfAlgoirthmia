use std::collections::{HashMap, VecDeque};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q06_p1.txt");
    let tree = Tree::new(lines);
    // RRLBFNDPQTXH@
    println!("part 1 = {}", tree.find_shortest().join(""));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q06_p2.txt");
    let tree = Tree::new(lines);
    println!(
        "part 2 = {}",
        tree.find_shortest()
            .iter()
            .map(|branch| branch.chars().next().unwrap())
            .collect::<String>()
    );
}

struct Tree {
    branches: HashMap<String, Vec<String>>,
}

impl Tree {
    fn new(lines: Vec<String>) -> Self {
        let mut branches = HashMap::new();
        for s in lines {
            let (left, right) = s.split_once(':').unwrap();
            let words = right.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
            branches.insert(left.into(), words);
        }

        Self { branches }
    }

    fn find_shortest(&self) -> Vec<String> {
        let mut lengths = HashMap::<usize, usize>::new();
        let mut queue = VecDeque::new();
        queue.push_back(("RR".to_string(), Vec::<String>::from(["RR".to_string()])));
        let mut solutions = Vec::new();
        while let Some((cur_node, path)) = queue.pop_front() {
            if let Some(branchlist) = self.branches.get(&cur_node) {
                for branch in branchlist {
                    if branch == "@" {
                        let mut solution = path.clone();
                        solution.push("@".to_string());
                        *lengths.entry(solution.len()).or_default() += 1;
                        solutions.push(solution);
                    } else {
                        let mut new_path = path.clone();
                        new_path.push(branch.clone());
                        queue.push_back((branch.to_string(), new_path));
                    }
                }
            }
        }

        let unique_length = lengths.iter().find(|(_, count)| **count == 1).unwrap().0;

        for s in solutions {
            if s.len() == *unique_length {
                return s;
            }
        }
        panic!("solution not found");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_tree() {
        let lines = aoclib::read_lines("test-input/test-part-1.txt");
        let tree = Tree::new(lines);
        assert_eq!(9, tree.branches.len());
    }

    #[test]
    fn test_solve_part_1() {
        let lines = aoclib::read_lines("test-input/test-part-1.txt");
        let tree = Tree::new(lines);
        assert_eq!(
            vec!["RR".to_string(), "B".to_string(), "@".to_string()],
            tree.find_shortest()
        );
    }
}
