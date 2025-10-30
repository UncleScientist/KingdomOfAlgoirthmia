use std::{collections::HashSet, path::Path};

fn main() {
    println!("part 1 = {}", part_1());
    println!("part 2 = {}", part_2().unwrap());
}

fn part_1() -> isize {
    // let map = Map::from_file("input/test_1.txt");
    let map = Map::from_file("input/everybody_codes_e2024_q20_p1.txt");

    let mut step = HashSet::new();
    step.insert(Glider::new(&map, (-1, 0), 1000));
    step.insert(Glider::new(&map, (0, 1), 1000));
    step.insert(Glider::new(&map, (1, 0), 1000));
    step.insert(Glider::new(&map, (0, -1), 1000));

    let mut heights = vec![vec![0isize; map.grid[0].len()]; map.grid.len()];
    let mut max = 0;

    let mut time = 0;
    while time <= 100 {
        let mut next_step = HashSet::new();
        for glider in step {
            if heights[glider.pos.0 as usize][glider.pos.1 as usize] > glider.altitude {
                continue;
            }
            heights[glider.pos.0 as usize][glider.pos.1 as usize] = glider.altitude;
            max = max.max(glider.altitude);
            for neighbor in glider.next_pos(&map) {
                next_step.insert(neighbor);
            }
        }
        step = next_step;
        time += 1;
    }

    max
}

fn part_2() -> Option<isize> {
    // let map = Map::from_file("input/test_2_3.txt");
    let map = Map::from_file("input/everybody_codes_e2024_q20_p2.txt");

    let mut step = HashSet::new();
    step.insert(Glider::new(&map, (-1, 0), 10000));
    step.insert(Glider::new(&map, (0, 1), 10000));
    step.insert(Glider::new(&map, (1, 0), 10000));
    step.insert(Glider::new(&map, (0, -1), 10000));

    let mut heights = vec![vec![(0isize, 0isize); map.grid[0].len()]; map.grid.len()];

    let mut time = 0;
    while !step.is_empty() {
        let mut next_step = HashSet::new();
        for glider in step {
            // println!("{time:3} | {glider:?}");
            if glider.found_start(&map) {
                return Some(time);
            }
            let (prev_visit, prev_height) = heights[glider.pos.0 as usize][glider.pos.1 as usize];
            if prev_visit > glider.visited && prev_height > glider.altitude {
                // println!("  -> skipping for higher altitude & visited");
                continue;
            }
            heights[glider.pos.0 as usize][glider.pos.1 as usize] = (glider.visited, glider.altitude);
            for neighbor in glider.next_pos(&map) {
                if neighbor.altitude < 9990 || neighbor.altitude > 11000 {
                    continue;
                }
                let (prev_visit, prev_height) = heights[neighbor.pos.0 as usize][neighbor.pos.1 as usize];
                if prev_visit > glider.visited && prev_height > glider.altitude {
                    continue;
                }

                next_step.insert(neighbor);
            }
        }
        step = next_step;
        time += 1;
    }

    None
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Glider {
    pos: (i64, i64),
    dir: (i64, i64),
    altitude: isize,
    visited: isize,
}

impl Glider {
    fn new(map: &Map, dir: (i64, i64), altitude: isize) -> Self {
        Glider {
            pos: map.get_start(),
            dir,
            altitude,
            visited: 0,
        }
    }

    fn next_pos(&self, map: &Map) -> Vec<Glider> {
        let mut result = Vec::new();
        let directions = if self.dir.0 == 0 {
            [(-1, 0), (1, 0), self.dir]
        } else {
            [(0, 1), (0, -1), self.dir]
        };
        for dir in directions {
            let mut visited = self.visited;
            let pos = (self.pos.0 + dir.0, self.pos.1 + dir.1);
            if !map.in_range(&pos) {
                continue;
            }
            if (map.at_loc_a(&pos) && visited != 0)
                || (map.at_loc_b(&pos) && visited != 1)
                || (map.at_loc_c(&pos) && visited != 2)
                || (map.at_start(&pos) && visited != 3)
            {
                continue;
            }

            visited += (map.at_loc_a(&pos) as isize)
                + (map.at_loc_b(&pos) as isize)
                + (map.at_loc_c(&pos) as isize);

            if let Some(dh) = map.delta_height(pos) {
                let altitude = dh + self.altitude;
                result.push(Glider {
                    pos,
                    dir,
                    altitude,
                    visited,
                });
            }
        }

        result
    }

    fn found_start(&self, map: &Map) -> bool {
        self.visited == 3 && self.altitude >= 10000 && map.at_start(&self.pos)
    }
}

// ---------------------------------------------------------------------------

#[derive(Debug)]
enum Item {
    Air,
    Downdraft,
    Updraft,
    Obstacle,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' | 'A' | 'B' | 'C' | 'S' => Self::Air,
            '-' => Self::Downdraft,
            '+' => Self::Updraft,
            '#' => Self::Obstacle,
            _ => panic!("invalid char '{value}' in input"),
        }
    }
}

struct Map {
    grid: Vec<Vec<Item>>,
    start: (usize, usize),
    loc_a: Option<(usize, usize)>,
    loc_b: Option<(usize, usize)>,
    loc_c: Option<(usize, usize)>,
}

impl Map {
    fn from_file<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
        let mut grid: Vec<Vec<Item>> = Vec::new();
        let mut start = None;
        let (mut loc_a, mut loc_b, mut loc_c) = (None, None, None);
        for (row, line) in lines.iter().enumerate() {
            let mut maprow = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                maprow.push(ch.into());
                match ch {
                    'S' => start = Some((row, col)),
                    'A' => loc_a = Some((row, col)),
                    'B' => loc_b = Some((row, col)),
                    'C' => loc_c = Some((row, col)),
                    _ => {}
                }
            }
            grid.push(maprow);
        }

        let Some(start) = start else {
            panic!("Starting location not found");
        };

        Self {
            grid,
            start,
            loc_a,
            loc_b,
            loc_c,
        }
    }

    fn in_range(&self, pos: &(i64, i64)) -> bool {
        !(pos.0 < 0
            || pos.1 < 0
            || pos.0 >= self.grid.len() as i64
            || pos.1 >= self.grid[0].len() as i64)
    }

    fn delta_height(&self, pos: (i64, i64)) -> Option<isize> {
        match self.grid[pos.0 as usize][pos.1 as usize] {
            Item::Air => Some(-1),
            Item::Downdraft => Some(-2),
            Item::Updraft => Some(1),
            Item::Obstacle => None,
        }
    }

    fn get_start(&self) -> (i64, i64) {
        (self.start.0 as i64, self.start.1 as i64)
    }

    fn _print(&self) {
        println!("start @ {:?}", self.start);
        println!("    A @ {:?}", self.loc_a);
        println!("    B @ {:?}", self.loc_b);
        println!("    C @ {:?}", self.loc_c);
        for line in &self.grid {
            println!("{line:?}");
        }
    }

    fn at_loc_a(&self, pos: &(i64, i64)) -> bool {
        if let Some(a) = self.loc_a
            && a == (pos.0 as usize, pos.1 as usize)
        {
            true
        } else {
            false
        }
    }

    fn at_loc_b(&self, pos: &(i64, i64)) -> bool {
        if let Some(b) = self.loc_b
            && b == (pos.0 as usize, pos.1 as usize)
        {
            true
        } else {
            false
        }
    }
    fn at_loc_c(&self, pos: &(i64, i64)) -> bool {
        if let Some(c) = self.loc_c
            && c == (pos.0 as usize, pos.1 as usize)
        {
            true
        } else {
            false
        }
    }

    fn at_start(&self, pos: &(i64, i64)) -> bool {
        self.start == (pos.0 as usize, pos.1 as usize)
    }
}
