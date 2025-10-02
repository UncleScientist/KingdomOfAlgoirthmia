use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2024_q15_p1.txt").expect("file");

    let mut maze = HashSet::<(i64, i64)>::new();
    let mut herbs = HashSet::<(i64, i64)>::new();
    let mut start = (0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let loc = (row as i64, col as i64);
            match ch {
                '#' => {}
                '.' => {
                    maze.insert(loc);
                    if row == 0 {
                        start = loc;
                    }
                }
                'H' => {
                    maze.insert(loc);
                    herbs.insert(loc);
                }
                _ => panic!("invalid character '{ch}'"),
            }
        }
    }

    println!(
        "part 1 = {}",
        search::bfs(
            &start,
            |pos| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .filter_map(|delta| {
                        let next = (pos.0 + delta.0, pos.1 + delta.1);
                        if maze.contains(&next) {
                            Some(next)
                        } else {
                            None
                        }
                    })
                    .collect()
            },
            |pos| herbs.contains(pos)
        )
        .unwrap()
            * 2
    );
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
