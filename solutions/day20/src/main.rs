use std::{collections::HashSet, path::Path};

fn main() {
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

    println!("part 1 = {max}");
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Glider {
    pos: (i64, i64),
    dir: (i64, i64),
    altitude: isize,
}

impl Glider {
    fn new(map: &Map, dir: (i64, i64), altitude: isize) -> Self {
        Glider {
            pos: map.get_start(),
            dir,
            altitude,
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
            let pos = (self.pos.0 + dir.0, self.pos.1 + dir.1);
            if !map.in_range(&pos) {
                continue;
            }
            if let Some(dh) = map.delta_height(pos) {
                let altitude = dh + self.altitude;
                result.push(Glider { pos, dir, altitude });
            }
        }

        result
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
            '.' | 'S' => Self::Air,
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
}

impl Map {
    fn from_file<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
        let mut grid: Vec<Vec<Item>> = Vec::new();
        let mut start = None;
        for (row, line) in lines.iter().enumerate() {
            let mut maprow = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                maprow.push(ch.into());
                if ch == 'S' {
                    start = Some((row, col));
                }
            }
            grid.push(maprow);
        }

        let Some(start) = start else {
            panic!("Starting location not found");
        };

        Self { grid, start }
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
        for line in &self.grid {
            println!("{line:?}");
        }
    }
}
