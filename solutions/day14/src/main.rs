use std::{collections::HashSet, str::FromStr};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q14_p1.txt");
    let steps: Vec<Step> = lines[0]
        .split(',')
        .map(|step| step.parse().unwrap())
        .collect();

    let mut max = 0;
    let mut pos = 0;
    for step in steps {
        pos += match step {
            Step::Up(up) => up,
            Step::Down(down) => -down,
            Step::Left(_) | Step::Right(_) | Step::Forward(_) | Step::Back(_) => 0,
        };
        max = max.max(pos);
    }
    println!("part 1 = {max}");

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q14_p2.txt");
    let _lines = ["U5,R3,D2,L5,U4,R5,D2", "U6,L1,D2,R3,U2,L1"];
    let trees: Vec<Vec<Step>> = lines
        .iter()
        .map(|line| line.split(',').map(|step| step.parse().unwrap()).collect())
        .collect();
    let mut segments = HashSet::new();
    for tree in trees {
        let mut pos = (0, 0, 0); // u/d, l/r, f/b

        for step in tree {
            let (amount, delta) = match step {
                Step::Up(up) => (up, (1, 0, 0)),
                Step::Down(down) => (down, (-1, 0, 0)),
                Step::Left(left) => (left, (0, -1, 0)),
                Step::Right(right) => (right, (0, 1, 0)),
                Step::Forward(forward) => (forward, (0, 0, 1)),
                Step::Back(back) => (back, (0, 0, -1)),
            };
            for _ in 0..amount {
                pos = (pos.0 + delta.0, pos.1 + delta.1, pos.2 + delta.2);
                segments.insert(pos);
            }
        }
    }
    println!("part 2 = {}", segments.len());
}

#[derive(Debug)]
enum Step {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
    Back(i64),
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().next().unwrap();
        let amount = s[1..].parse().unwrap();
        Ok(match dir {
            'U' => Self::Up(amount),
            'D' => Self::Down(amount),
            'L' => Self::Left(amount),
            'R' => Self::Right(amount),
            'F' => Self::Forward(amount),
            'B' => Self::Back(amount),
            _ => panic!("bad char '{dir}'"),
        })
    }
}
