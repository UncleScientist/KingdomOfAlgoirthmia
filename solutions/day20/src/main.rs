use std::{collections::HashMap, path::Path};

fn main() {
    println!("part 1 = {}", part_1().unwrap());
    println!("part 2 = {}", part_2().unwrap());
    println!("part 3 = {}", part_3().unwrap());
}

fn part_1() -> Option<isize> {
    // let map = Map::from_file("input/test_1.txt");
    let map = Map::from_file("input/everybody_codes_e2024_q20_p1.txt");

    let mut step = HashMap::new();
    step.insert(Glider::new(&map, (-1, 0)), 1000);
    step.insert(Glider::new(&map, (0, 1)), 1000);
    step.insert(Glider::new(&map, (1, 0)), 1000);
    step.insert(Glider::new(&map, (0, -1)), 1000);

    let mut time = 0;
    while time < 100 {
        let mut next_step = step.clone();
        for (glider, altitude) in step {
            for (next_state, next_alt) in glider.next_pos(altitude, &map, Style::Stop) {
                let cur_alt = next_step.entry(next_state.clone()).or_insert(next_alt);
                if next_alt >= *cur_alt {
                    *cur_alt = next_alt;
                }
            }
        }
        step = next_step;
        time += 1;
    }

    step.values().max().copied()
}

fn part_2() -> Option<isize> {
    // let map = Map::from_file("input/test_2_3.txt");
    let map = Map::from_file("input/everybody_codes_e2024_q20_p2.txt");

    let mut search = HashMap::<Glider, isize>::from([
        (Glider::new(&map, (-1, 0)), 10000),
        (Glider::new(&map, (0, 1)), 10000),
        (Glider::new(&map, (1, 0)), 10000),
        (Glider::new(&map, (0, -1)), 10000),
    ]);

    let mut time = 0;
    while !search.is_empty() {
        let mut next_search = search.clone();
        for (glider_state, altitude) in search {
            if altitude >= 10000 && glider_state.found_start(&map, 3) {
                return Some(time);
            }
            for (next_state, next_alt) in glider_state.next_pos(altitude, &map, Style::Stop) {
                let cur_alt = next_search.entry(next_state.clone()).or_insert(next_alt);
                if next_alt >= *cur_alt {
                    *cur_alt = next_alt;
                }
            }
        }
        search = next_search;
        time += 1;
    }

    None
}

fn part_3() -> Option<isize> {
    // let map = Map::from_file("input/test_3.txt");
    let map = Map::from_file("input/everybody_codes_e2024_q20_p3.txt");

    let mut smallest_col = None;
    let mut dist_to_start = map.grid[0].len();
    let mut min_loss = isize::MIN;
    for col in 0..map.grid[0].len() {
        let mut cur_alt = 0;
        for row in 0..map.grid.len() {
            if let Some(delta) = map.delta_height((row as i64, col as i64)) {
                cur_alt += delta;
            } else {
                cur_alt = isize::MIN;
                break;
            }
        }
        if cur_alt != isize::MIN {
            let dist = col.abs_diff(map.start.1);
            if cur_alt > min_loss || (cur_alt == min_loss && dist < dist_to_start) {
                min_loss = cur_alt;
                smallest_col = Some(col);
                dist_to_start = dist;
            }
        }
    }

    let Some(smallest_col) = smallest_col else {
        panic!("can't find a col to fly down");
    };

    let starting_altitude = 384400;
    let mut search = HashMap::<Glider, isize>::from([
        (Glider::new(&map, (1, 0)), starting_altitude),
        (Glider::new(&map, (0, -1)), starting_altitude),
        (Glider::new(&map, (0, 1)), starting_altitude),
    ]);

    let mut best_so_far = None;
    while !search.is_empty() {
        let mut next_search = HashMap::new();
        for (glider_state, altitude) in search.iter() {
            if *altitude == 0 {
                best_so_far = match best_so_far {
                    Some(b) if b < glider_state.pos.0 as isize => Some(glider_state.pos.0 as isize),
                    None => Some(glider_state.pos.0 as isize),
                    _ => best_so_far
                };
            }
            for (next_state, next_alt) in glider_state.next_pos(*altitude, &map, Style::Wrap) {
                if next_alt < 0 {
                    continue;
                }
                if next_state.pos.1 <= glider_state.pos.1
                    && glider_state.pos.1 < smallest_col as i64
                {
                    continue;
                }
                if next_state.pos.1 >= glider_state.pos.1
                    && glider_state.pos.1 > smallest_col as i64
                {
                    continue;
                }
                if let Some(alt) = search.get(&next_state) {
                    if *alt < next_alt {
                        next_search.insert(next_state, next_alt);
                    } else {
                        next_search.insert(next_state, *alt);
                    }
                } else {
                    next_search.insert(next_state, next_alt);
                }
            }
        }
        search = next_search;
    }

    if let Some(b) = best_so_far {
        return Some(b);
    }

    println!("took {dist_to_start} to get: min_loss = {min_loss} at column {smallest_col:?}");
    println!("height is {}", map.grid.len());

    let start_height = (384400 - dist_to_start - 1) as isize;
    let grids_needed = start_height / (-min_loss);
    dbg!(grids_needed);
    let mut dist_flown = grids_needed * map.grid.len() as isize;
    dbg!(dist_flown);
    let final_height = start_height % (-min_loss);
    dbg!(final_height);

    let grids_flown = dist_flown / map.grid.len() as isize;
    let mut latest_altitude = start_height - (grids_flown * (-min_loss));

    let mut row = 1;
    dist_flown += 1;
    while latest_altitude > 0 {
        let dh = map.delta_height((row, smallest_col as i64)).unwrap();
        latest_altitude += dh;
        dist_flown += 1;
        println!("row {row}, dh {dh}, {latest_altitude}, {dist_flown}");
        row += 1;
    }

    println!("{start_height}, {dist_flown}, {grids_flown}, {latest_altitude}");

    Some(dist_flown)
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Glider {
    pos: (i64, i64),
    dir: (i64, i64),
    visited: isize,
}

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Wrap,
    Stop,
}

impl Glider {
    fn new(map: &Map, dir: (i64, i64)) -> Self {
        Glider {
            pos: map.get_start(),
            dir,
            visited: 0,
        }
    }

    fn found_start(&self, map: &Map, visited: isize) -> bool {
        self.visited == visited && map.at_start(&self.pos)
    }

    fn next_pos(&self, cur_altitude: isize, map: &Map, style: Style) -> Vec<(Glider, isize)> {
        let mut result = Vec::new();
        let directions = if self.dir.0 == 0 {
            [(-1, 0), (1, 0), self.dir]
        } else {
            [(0, 1), (0, -1), self.dir]
        };
        for dir in directions {
            let mut visited = self.visited;
            let pos = (self.pos.0 + dir.0, self.pos.1 + dir.1);
            if !map.in_range(&pos, style) {
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
                result.push((Glider { pos, dir, visited }, dh + cur_altitude));
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

    fn in_range(&self, pos: &(i64, i64), style: Style) -> bool {
        !(pos.0 < 0
            || pos.1 < 0
            || (style == Style::Stop && pos.0 >= self.grid.len() as i64)
            || pos.1 >= self.grid[0].len() as i64)
    }

    fn delta_height(&self, pos: (i64, i64)) -> Option<isize> {
        match self.grid[pos.0 as usize % self.grid.len()][pos.1 as usize] {
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
