use std::path::Path;

fn main() {
    let mut grid = Grid::from_file("input/everybody_codes_e2024_q19_p1.txt");
    // let mut grid = Grid::from_file("input/test_1.txt");
    grid.apply_rotations();
    println!("part 1:");
    grid._print();

    let mut grid = Grid::from_file("input/everybody_codes_e2024_q19_p2.txt");
    // let mut grid = Grid::from_file("input/test_2.txt");
    for _ in 0..100 {
        grid.apply_rotations();
    }
    println!("part 2:");
    grid._print();
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl From<char> for Rotation {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("invalid rotation '{value}'"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    seq: Vec<Rotation>,
    message: Vec<Vec<char>>,
}

impl Grid {
    fn from_file<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);

        let mut message = Vec::new();
        let seq = lines[0].chars().map(|ch| ch.into()).collect();
        for line in &lines[1..] {
            message.push(line.chars().collect::<Vec<_>>());
        }
        Self { message, seq }
    }

    fn _print(&self) {
        for row in &self.message {
            println!("{}", row.iter().copied().collect::<String>());
        }
    }

    fn apply_rotations(&mut self) {
        let mut cur_seq = 0;
        for row in 1..self.message.len() - 1 {
            for col in 1..self.message[0].len() - 1 {
                let (row, col) = (row as i64, col as i64);
                match self.seq[cur_seq] {
                    Rotation::Left => {
                        let save = self.message[(row + ROT_LEFT[0].0) as usize]
                            [(col + ROT_LEFT[0].1) as usize];
                        for delta in 1..8 {
                            self.message[(row + ROT_LEFT[delta - 1].0) as usize]
                                [(col + ROT_LEFT[delta - 1].1) as usize] = self.message
                                [(row + ROT_LEFT[delta].0) as usize]
                                [(col + ROT_LEFT[delta].1) as usize]
                        }
                        self.message[(row + ROT_LEFT[7].0) as usize]
                            [(col + ROT_LEFT[7].1) as usize] = save;
                    }

                    Rotation::Right => {
                        let save = self.message[(row + ROT_RIGHT[0].0) as usize]
                            [(col + ROT_RIGHT[0].1) as usize];
                        for delta in 1..8 {
                            self.message[(row + ROT_RIGHT[delta - 1].0) as usize]
                                [(col + ROT_RIGHT[delta - 1].1) as usize] = self.message
                                [(row + ROT_RIGHT[delta].0) as usize]
                                [(col + ROT_RIGHT[delta].1) as usize]
                        }
                        self.message[(row + ROT_RIGHT[7].0) as usize]
                            [(col + ROT_RIGHT[7].1) as usize] = save;
                    }
                }

                cur_seq = (cur_seq + 1) % self.seq.len();
            }
        }
    }
}

const ROT_LEFT: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];
const ROT_RIGHT: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];
