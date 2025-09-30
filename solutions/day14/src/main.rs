use std::str::FromStr;

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
}

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
