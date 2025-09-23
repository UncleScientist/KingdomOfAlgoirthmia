use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q13_p1.txt");
    let grid = Grid::parse(&lines);
    // grid._print();
    println!("part 1 = {}", grid.dijkstra_search().unwrap());
}

#[derive(Debug)]
struct Grid {
    grid: HashMap<(i64, i64), i64>,
    start: (i64, i64),
    end: (i64, i64),
    _rows: usize,
    _cols: usize,
}

impl Grid {
    fn _print(&self) {
        for row in 0..self._rows {
            for col in 0..self._cols {
                if let Some(val) = self.grid.get(&(row as i64, col as i64)) {
                    print!("{val}");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn parse<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut grid = HashMap::new();
        let mut start = None;
        let mut end = None;
        let rows = lines.len();
        let cols = lines[0].as_ref().len();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.as_ref().chars().enumerate() {
                let pos = (row as i64, col as i64);
                if let Some(level) = match ch {
                    'S' | 'E' => {
                        if ch == 'S' {
                            start = Some(pos);
                        } else {
                            end = Some(pos);
                        }
                        Some(0)
                    }
                    '0'..='9' => Some((ch as u8 - b'0') as i64),
                    '#' | ' ' => None,
                    _ => panic!("Illegal char '{ch}'"),
                } {
                    grid.insert(pos, level);
                }
            }
        }

        Self {
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
            _rows: rows,
            _cols: cols,
        }
    }

    fn cost(level1: i64, level2: i64) -> i64 {
        let l1 = level1.min(level2);
        let l2 = level1.max(level2);
        if l2 - l1 > 5 {
            10 + l1 - l2
        } else {
            l2 - l1
        }
    }

    fn neighbors(&self, pos: &(i64, i64)) -> Vec<((i64, i64), i64)> {
        let mut result = Vec::new();
        let level = self.grid[pos];
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(new_level) = self.grid.get(&new_pos) {
                let min_dist = Self::cost(level, *new_level);
                result.push((new_pos, min_dist + 1));
            }
        }
        result
    }

    fn dijkstra_search(&self) -> Option<i64> {
        let mut queue = BTreeSet::from([(0, self.start)]);
        let mut visited = HashSet::new();
        let mut dist = HashMap::new();

        dist.insert(self.start, 0);

        while let Some((time, pos)) = queue.pop_first() {
            if pos == self.end {
                return Some(time);
            }
            if visited.insert(pos) {
                for (new_pos, cost) in self.neighbors(&pos) {
                    let new_time = time + cost;
                    let old_time = dist.entry(new_pos).or_insert(i64::MAX);
                    if new_time < *old_time {
                        dist.insert(new_pos, new_time);
                        queue.insert((new_time, new_pos));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbors() {
        let lines = ["#######", "#6769##", "S50505E", "#97434#", "#######"];
        let grid = Grid::parse(&lines);
        let neighbors = grid.neighbors(&(2, 2));
        assert_eq!(
            vec![((2, 3), 6), ((3, 2), 4), ((2, 1), 6), ((1, 2), 4)],
            neighbors
        );
    }

    #[test]
    fn test_wrapping_neighbors() {
        let lines = ["#######", "#6769##", "S50505E", "#97434#", "#######"];
        let grid = Grid::parse(&lines);
        let neighbors = grid.neighbors(&(2, 4));
        assert_eq!(
            vec![((2, 5), 6), ((3, 4), 4), ((2, 3), 6), ((1, 4), 2)],
            neighbors
        );
    }

    #[test]
    fn test_end_neighbors() {
        let lines = ["#######", "#6769##", "S50505E", "#97434#", "#######"];
        let grid = Grid::parse(&lines);
        let neighbors = grid.neighbors(&(2, 5));
        assert_eq!(vec![((2, 6), 6), ((3, 5), 2), ((2, 4), 6)], neighbors);
    }

    #[test]
    fn test_search() {
        let lines = ["#######", "#6769##", "S50505E", "#97434#", "#######"];
        let grid = Grid::parse(&lines);
        assert_eq!(Some(28), grid.dijkstra_search());
    }

    #[test]
    fn test_cost() {
        assert_eq!(0, Grid::cost(4, 4));
        assert_eq!(1, Grid::cost(4, 5));
        assert_eq!(1, Grid::cost(5, 4));
        assert_eq!(1, Grid::cost(0, 9));
        assert_eq!(1, Grid::cost(9, 0));
    }
}
