use std::collections::HashMap;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p1.txt");
    // let lines = aoclib::read_lines("input/test-1.txt");
    let mut targets = Grid::new(&lines);
    println!("part 1 = {}", targets.attack());

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p2.txt");
    let mut targets = Grid::new(&lines);
    println!("part 2 = {}", targets.attack());

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p3.txt");
    let coords: Vec<(i64, i64)> = lines
        .iter()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let mut grid = PowerRank::default();
    grid.generate_up_to(4000);
    println!("part 3 = {}", grid.rank_power(&coords));
}

#[derive(Debug)]
enum Target {
    Soft,
    Hard,
}

impl From<char> for Target {
    fn from(value: char) -> Self {
        match value {
            'T' => Self::Soft,
            'H' => Self::Hard,
            _ => panic!("invalid target '{value}'"),
        }
    }
}

#[derive(Debug, Default)]
struct Grid {
    catapults: Vec<(i64, i64, i64)>,
    pos: HashMap<(i64, i64), Target>,
    max_col: i64,
    ground: i64,
}

impl Grid {
    fn new(lines: &[String]) -> Self {
        let mut ground = 0;
        let mut catapults = Vec::new();
        let mut pos = HashMap::new();
        let mut max_col = 0;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'A' | 'B' | 'C' => {
                        catapults.push(((ch as u8 - b'A' + 1) as i64, row as i64, col as i64));
                    }
                    'T' | 'H' => {
                        pos.insert((row as i64, col as i64), ch.into());
                        max_col = max_col.max(col as i64 + 1);
                    }
                    '.' => {}
                    '=' => {
                        ground = row as i64;
                    }
                    _ => panic!("invalid char '{ch}'"),
                }
            }
        }
        Self {
            catapults,
            pos,
            max_col,
            ground,
        }
    }

    fn attack(&mut self) -> i64 {
        let catapults = self.catapults.clone();

        let mut result = 0;
        while !self.pos.is_empty() {
            for (id, start_row, start_col) in &catapults {
                let mut power = 1;
                while self.in_range(power * 2 + start_col) {
                    let mut cur_row = start_row - power + 1;
                    let mut cur_col = power * 2 + start_col + 1;
                    while cur_row < self.ground {
                        if self.hits(cur_row, cur_col) {
                            result += *id * power;
                            break;
                        }
                        cur_row += 1;
                        cur_col += 1;
                    }

                    power += 1;
                }
            }
        }

        result
    }

    fn hits(&mut self, row: i64, col: i64) -> bool {
        if self.pos.contains_key(&(row, col)) && !self.pos.contains_key(&(row - 1, col)) {
            if let Some(Target::Hard) = self.pos.remove(&(row, col)) {
                self.pos.insert((row, col), Target::Soft);
            }
            return true;
        }

        false
    }

    fn in_range(&self, col: i64) -> bool {
        col < self.max_col
    }
}

#[derive(Debug, Default)]
struct PowerRank {
    score: HashMap<(i64, i64), (i64, i64)>,
    rows: i64,
    cols: i64,
}

impl PowerRank {
    fn calc_power(&mut self, start_row: i64, start_col: i64, power: i64) {
        let rank_score = (start_row + 1) * power;

        let mut row = start_row;
        let mut col = start_col;
        let mut time = 0;

        for _ in 0..power {
            time += 1;
            row += 1;
            col += 1;
            let entry = self.score.entry((row, col)).or_insert((i64::MAX, time));
            if rank_score < entry.0 {
                *entry = (rank_score, time);
            }
        }

        self.rows = self.rows.max(row + 1);

        for _ in 0..power {
            time += 1;
            col += 1;
            let entry = self.score.entry((row, col)).or_insert((i64::MAX, time));
            if rank_score < entry.0 {
                *entry = (rank_score, time);
            }
        }

        while row > 0 {
            time += 1;
            row -= 1;
            col += 1;
            let entry = self.score.entry((row, col)).or_insert((i64::MAX, time));
            if rank_score < entry.0 {
                *entry = (rank_score, time);
            }
        }
        self.cols = self.cols.max(col + 1);
    }

    fn _print(&self) {
        println!("{}, {}", self.rows, self.cols);

        for row in 0..self.rows {
            let real_row = self.rows - row - 1;
            print!("{real_row:2} | ");
            for col in 0..self.cols {
                if col == 0 {
                    match real_row {
                        0 => print!("...A..."),
                        1 => print!("...B..."),
                        2 => print!("...C..."),
                        _ => print!("       "),
                    }
                } else if let Some(val) = self.score.get(&(real_row, col)) {
                    print!("{:3}/{:<3}", val.0, val.1);
                } else {
                    print!("       ");
                }
            }
            println!();
        }
    }

    fn generate_up_to(&mut self, limit: i64) {
        for power in 1..limit {
            self.calc_power(0, 0, power);
            self.calc_power(1, 0, power);
            self.calc_power(2, 0, power);
        }
    }

    fn shot_at_time(&self, mut coord: (i64, i64), mut time: i64) -> Option<i64> {
        // println!("time {time}, meteor starts at {coord:?}");
        while coord.0 > 0 {
            time += 1;
            coord.0 -= 1;
            coord.1 -= 1;
            if let Some(entry) = self.score.get(&coord) {
                // println!("{coord:?} / {time} -> {entry:?}");
                if entry.1 == time {
                    return Some(entry.0);
                } else if entry.1 < time {
                    break;
                }
            }
        }

        None
    }

    fn rank_power(&self, coords: &[(i64, i64)]) -> i64 {
        let mut total = 0;
        for loc in coords.iter() {
            let mut time = 0;
            total += loop {
                if let Some(found) = self.shot_at_time((loc.1, loc.0), time) {
                    break found;
                } else {
                    time -= 1;
                }
                if loc.1 < -time {
                    panic!("not found");
                }
            };
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_2() {
        let lines = [(6, 5), (6, 7), (10, 5)];

        let mut grid = PowerRank::default();
        grid.generate_up_to(6);
        assert_eq!(11, grid.rank_power(&lines));
    }

    #[test]
    fn test_example_3() {
        let lines = [(5, 5)];
        let mut grid = PowerRank::default();
        grid.generate_up_to(6);
        grid._print();
        assert_eq!(2, grid.rank_power(&lines));
    }
}
