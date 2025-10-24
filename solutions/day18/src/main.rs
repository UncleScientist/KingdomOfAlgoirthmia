use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() {
    let mut garden = Garden::read_from_file("input/everybody_codes_e2024_q18_p1.txt");
    println!("part 1 = {}", garden.time_to_water_trees());
    let mut garden = Garden::read_from_file("input/everybody_codes_e2024_q18_p2.txt");
    println!("part 2 = {}", garden.time_to_water_trees());

    let garden = Garden::read_from_file("input/everybody_codes_e2024_q18_p3.txt");
    // let garden = Garden::read_from_file("input/test_3.txt");
    println!("palm tree count = {}", garden.find_optimal_spot());
}

struct Garden {
    maze: HashSet<(i64, i64)>,
    palms: HashSet<(i64, i64)>,
    start: Vec<(i64, i64)>,
}

impl Garden {
    fn read_from_file<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
        let last_col = (lines[0].len() - 1) as i64;

        let mut maze = HashSet::<(i64, i64)>::new();
        let mut palms = HashSet::<(i64, i64)>::new();
        let mut start = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let (row, col) = (row as i64, col as i64);
                match ch {
                    '#' => {}
                    '.' => {
                        maze.insert((row, col));
                        if col == 0 || col == last_col {
                            start.push((row, col));
                        }
                    }
                    'P' => {
                        maze.insert((row, col));
                        palms.insert((row, col));
                    }
                    _ => {
                        panic!("invalid char '{ch}'");
                    }
                }
            }
        }

        Self { maze, palms, start }
    }

    fn time_to_water_trees(&mut self) -> usize {
        let mut step = HashSet::from_iter(self.start.clone());
        let mut visited = HashSet::new();
        let mut time = 0;
        while !self.palms.is_empty() {
            let mut next_step = HashSet::new();
            for pos in step {
                if visited.insert(pos) {
                    self.palms.remove(&pos);
                    for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                        let newpos = (pos.0 + delta.0, pos.1 + delta.1);
                        if self.maze.contains(&newpos) {
                            next_step.insert(newpos);
                        }
                    }
                }
            }
            step = next_step;
            if !self.palms.is_empty() {
                time += 1;
            }
        }

        time
    }

    fn find_optimal_spot(&self) -> usize {
        let mut found = HashMap::<(i64, i64), usize>::new();
        let mut step = HashSet::<((i64, i64), (i64, i64))>::from_iter(
            self.palms.iter().map(|palm| (*palm, *palm)),
        );
        let mut visited = HashSet::new();
        let mut time = 0;
        loop {
            let mut next_step = HashSet::new();

            for (palm, pos) in step {
                if visited.insert((palm, pos)) {
                    *found.entry(pos).or_default() += time;

                    for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                        let newpos = (pos.0 + delta.0, pos.1 + delta.1);
                        if self.maze.contains(&newpos) {
                            next_step.insert((palm, newpos));
                        }
                    }
                }
            }

            if next_step.is_empty() {
                break;
            }

            time += 1;
            step = next_step;
        }

        let mut min = usize::MAX;
        for (_, &amount) in found.iter().filter(|(pos, _)| !self.palms.contains(pos)) {
            min = min.min(amount);
        }

        min
    }
}
