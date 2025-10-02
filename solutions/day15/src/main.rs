use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2024_q15_p1.txt").expect("file");
    let garden = Garden::parse(&data);
    println!("part 1 = {}", garden.search_any().unwrap() * 2);

    let data = std::fs::read_to_string("input/everybody_codes_e2024_q15_p2.txt").expect("file");
    //let data = std::fs::read_to_string("input/test_2.txt").expect("file");
    let garden = Garden::parse(&data);
    println!("part 2 = {}", garden.search_all().unwrap());
}

struct Garden {
    maze: HashSet<(i64, i64)>,
    herb_types: HashSet<char>,
    herbs: HashMap<(i64, i64), char>,
    start: (i64, i64),
}

impl Garden {
    fn parse(data: &str) -> Self {
        let mut maze = HashSet::<(i64, i64)>::new();
        let mut herb_types = HashSet::<char>::new();
        let mut herbs = HashMap::<(i64, i64), char>::new();
        let mut start = (0, 0);

        for (row, line) in data.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let loc = (row as i64, col as i64);
                match ch {
                    '#' | '~' => {}
                    '.' => {
                        maze.insert(loc);
                        if row == 0 {
                            start = loc;
                        }
                    }
                    'A'..='Z' => {
                        maze.insert(loc);
                        herbs.insert(loc, ch);
                        herb_types.insert(ch);
                    }
                    _ => panic!("invalid character '{ch}'"),
                }
            }
        }

        Self {
            maze,
            herbs,
            herb_types,
            start,
        }
    }

    fn search_any(&self) -> Option<usize> {
        search::bfs(
            &self.start,
            |pos| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter_map(|delta| {
                        let next = (pos.0 + delta.0, pos.1 + delta.1);
                        if self.maze.contains(&next) {
                            Some(next)
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            |pos| self.herbs.contains_key(pos),
        )
    }

    fn search_all(&self) -> Option<usize> {
        let herbs_remaining = (1u64 << self.herb_types.len()) - 1;
        let start_state = (herbs_remaining, self.start);

        search::bfs(
            &start_state,
            |(remaining, pos)| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter_map(|delta| {
                        let next = (pos.0 + delta.0, pos.1 + delta.1);
                        if self.maze.contains(&next) {
                            let next_remaining = if let Some(h) = self.herbs.get(&next) {
                                let bit = 1u64 << (*h as u8 - b'A');
                                remaining & !bit
                            } else {
                                *remaining
                            };
                            Some((next_remaining, next))
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            |(remaining, pos)| *remaining == 0 && *pos == self.start,
        )
    }
}

mod search {
    use std::{
        collections::{HashSet, VecDeque},
        hash::Hash,
    };

    pub fn bfs<T>(
        start: &T,
        neighbors: impl Fn(&T) -> Vec<T>,
        is_end: impl Fn(&T) -> bool,
    ) -> Option<usize>
    where
        T: Clone + Hash + Eq,
    {
        let mut queue = VecDeque::from([(0, start.clone())]);
        let mut visited = HashSet::new();

        while let Some((dist, entry)) = queue.pop_front() {
            if is_end(&entry) {
                return Some(dist);
            }
            if visited.insert(entry.clone()) {
                for neighbor in neighbors(&entry) {
                    queue.push_back((dist + 1, neighbor));
                }
            }
        }

        None
    }
}
