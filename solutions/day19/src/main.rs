use std::{ops::Add, path::Path};

fn main() {
    println!("{}", mba(12.34, 20));
    let mut grid = Grid::from_file("input/everybody_codes_e2024_q19_p1.txt");
    // let mut grid = Grid::from_file("input/test_1.txt");
    apply_rotations(&grid.seq, &mut grid.message);
    println!("part 1 = {}", grid.extract_value().unwrap());

    let mut grid = Grid::from_file("input/everybody_codes_e2024_q19_p2.txt");
    // let mut grid = Grid::from_file("input/test_2.txt");
    grid.multiply_by(100);
    println!("part 2 = {}", grid.extract_value().unwrap());

    let mut grid = Grid::from_file("input/everybody_codes_e2024_q19_p3.txt");
    grid.multiply_by(1048576000);
    println!("part 3 = {}", grid.extract_value().unwrap());
}

#[derive(Clone)]
struct Mapping(Vec<Vec<(usize, usize)>>);

impl Add for Mapping {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mapping(
            (0..self.0.len())
                .map(|row| {
                    (0..self.0[0].len())
                        .map(|col| {
                            let (maprow, mapcol) = rhs.0[row][col];
                            self.0[maprow][mapcol]
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
    }
}

// data x multiplier
// where data is what you want to multiply, and multiplier is very large
//       data has addition defined for it
//
// mba(data, multiplier) -> data
//  -> if multiplier is even: mba(data + data, multiplier / 2)
//  -> if multiplier is odd : data + mba(data, multiplier - 1)

fn mba<A: Clone + Add<Output = A>>(data: A, multiplier: usize) -> A {
    match multiplier {
        1 => data,
        n if n.is_multiple_of(2) => mba(data.clone() + data, n / 2),
        n => data.clone() + mba(data, n - 1),
    }
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

    fn multiply_by(&mut self, multiplier: usize) {
        let mut increment = (0..self.message.len())
            .map(|row| {
                (0..self.message[0].len())
                    .map(|col| (row, col))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        apply_rotations(&self.seq, &mut increment);

        let map = mba(Mapping(increment), multiplier);
        let orig = self.message.clone();
        for row in 0..self.message.len() {
            for col in 0..self.message[0].len() {
                let pos = map.0[row][col];
                self.message[row][col] = orig[pos.0][pos.1];
            }
        }
    }

    fn _print(&self) {
        for row in &self.message {
            println!("{}", row.iter().copied().collect::<String>());
        }
    }

    fn extract_value(&self) -> Option<String> {
        for row in 0..self.message.len() {
            for col in 0..self.message[0].len() {
                if self.message[row][col] == '>' {
                    let mut end = col + 1;
                    let mut result = String::from("");
                    while self.message[row][end] != '<' {
                        result.push(self.message[row][end]);
                        end += 1;
                    }
                    return Some(result);
                }
            }
        }

        None
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

fn apply_rotations<T: Copy>(seq: &[Rotation], message: &mut [Vec<T>]) {
    let mut cur_seq = 0;
    for row in 1..message.len() - 1 {
        for col in 1..message[0].len() - 1 {
            let (row, col) = (row as i64, col as i64);
            match seq[cur_seq] {
                Rotation::Left => {
                    let save =
                        message[(row + ROT_LEFT[0].0) as usize][(col + ROT_LEFT[0].1) as usize];
                    for delta in 1..8 {
                        message[(row + ROT_LEFT[delta - 1].0) as usize]
                            [(col + ROT_LEFT[delta - 1].1) as usize] = message
                            [(row + ROT_LEFT[delta].0) as usize]
                            [(col + ROT_LEFT[delta].1) as usize]
                    }
                    message[(row + ROT_LEFT[7].0) as usize][(col + ROT_LEFT[7].1) as usize] = save;
                }

                Rotation::Right => {
                    let save =
                        message[(row + ROT_RIGHT[0].0) as usize][(col + ROT_RIGHT[0].1) as usize];
                    for delta in 1..8 {
                        message[(row + ROT_RIGHT[delta - 1].0) as usize]
                            [(col + ROT_RIGHT[delta - 1].1) as usize] = message
                            [(row + ROT_RIGHT[delta].0) as usize]
                            [(col + ROT_RIGHT[delta].1) as usize]
                    }
                    message[(row + ROT_RIGHT[7].0) as usize][(col + ROT_RIGHT[7].1) as usize] =
                        save;
                }
            }

            cur_seq = (cur_seq + 1) % seq.len();
        }
    }
}
