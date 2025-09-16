use std::collections::HashMap;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p1.txt");
    // let lines = aoclib::read_lines("input/test-1.txt");
    let mut targets = Grid::new(&lines);
    println!("part 1 = {}", targets.attack());

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p2.txt");
    let mut targets = Grid::new(&lines);
    println!("part 2 = {}", targets.attack());
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
