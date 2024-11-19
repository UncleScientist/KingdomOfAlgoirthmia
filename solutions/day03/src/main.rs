use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q03_p1.txt");

    let mut map = Map::new(&lines);
    let mut total = 0;
    loop {
        let count = map.earth.len();
        if count == 0 {
            break;
        }
        total += count;
        map.step();
    }
    println!("part 1 = {total}");
}

#[derive(Debug)]
struct Map {
    earth: HashSet<(i64, i64)>,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let mut earth = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    earth.insert((row as i64, col as i64));
                }
            }
        }
        Self { earth }
    }

    fn step(&mut self) {
        let new_earth = self
            .earth
            .iter()
            .filter_map(|(row, col)| {
                if self.earth.contains(&(*row - 1, *col))
                    && self.earth.contains(&(*row + 1, *col))
                    && self.earth.contains(&(*row, *col - 1))
                    && self.earth.contains(&(*row, *col + 1))
                {
                    Some((*row, *col))
                } else {
                    None
                }
            })
            .collect::<HashSet<(i64, i64)>>();
        self.earth = new_earth;
    }
}
