use std::{path::Path, str::FromStr};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q07_p1.txt");
    let mut scores = Vec::new();
    for line in lines {
        let plan: Plan = line.parse().unwrap();
        scores.push((plan.name, plan.execute(10)));
    }
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "part 1 = {}",
        scores.iter().map(|score| score.0).collect::<String>()
    );

    let mut scores = Vec::new();
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q07_p2.txt");
    let plans: Vec<Plan> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let track = Track::init("input/track.txt");
    for plan in plans {
        scores.push((plan.name, track.execute_plan(&plan, 10)));
    }

    scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "part 2 = {}",
        scores.iter().map(|score| score.0).collect::<String>()
    );
}

#[derive(Debug)]
enum Action {
    Increase,
    Decrease,
    Equal,
    StartEnd,
}

impl From<char> for Action {
    fn from(value: char) -> Self {
        match value {
            '+' => Self::Increase,
            '-' => Self::Decrease,
            '=' => Self::Equal,
            'S' => Self::StartEnd,
            _ => panic!("Invalid Action: '{value}'"),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Increase),
            "-" => Ok(Self::Decrease),
            "=" => Ok(Self::Equal),
            "S" => Ok(Self::StartEnd),
            _ => Err(format!("Invalid Action: '{s}'")),
        }
    }
}

#[derive(Debug)]
struct Plan {
    name: char,
    seq: Vec<Action>,
}

impl FromStr for Plan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn action(s: &str) -> Action {
            s.parse().unwrap()
        }

        let (name, seq) = s.split_once(':').unwrap();
        let name = name.chars().next().unwrap();

        let seq = seq.split(',').map(action).collect();
        Ok(Self { name, seq })
    }
}

impl Plan {
    fn execute(&self, rounds: usize) -> usize {
        let mut cur = 10;
        let mut sum = 0;

        for round in 0..rounds {
            match self.seq[round % self.seq.len()] {
                Action::Increase => cur += 1,
                Action::Decrease if cur > 0 => cur -= 1,
                _ => {}
            }
            sum += cur;
        }

        sum
    }

    fn update(&self, index: usize, cur: usize) -> usize {
        match self.seq[index % self.seq.len()] {
            Action::Increase => cur + 1,
            Action::Decrease if cur > 0 => cur - 1,
            _ => cur,
        }
    }
}

#[derive(Debug)]
struct Track {
    track: Vec<Action>,
}

impl Track {
    fn init<P: AsRef<Path>>(path: P) -> Self {
        let mut track: Vec<Action> = Vec::new();

        let lines = aoclib::read_lines(path);

        // top line
        track.extend(lines[0].chars().map(|s| s.into()));

        // right side
        for line in lines.iter().take(lines.len() - 1).skip(1) {
            let last_char = line.chars().next_back().unwrap();
            track.push(last_char.into());
        }

        // bottom (reverse)
        track.extend(lines[lines.len() - 1].chars().rev().map(|s| s.into()));

        // left side (bottom up)
        for index in (1..lines.len() - 1).rev() {
            let last_char = lines[index].chars().next().unwrap();
            track.push(last_char.into());
        }

        Self { track }
    }

    fn execute_plan(&self, plan: &Plan, loops: usize) -> usize {
        let track_len = self.track.len();

        let mut cur = 10;
        let mut sum = 0;

        for index in 0..loops * track_len {
            match self.track[(index + 1) % track_len] {
                Action::Increase => cur += 1,
                Action::Decrease if cur > 0 => cur -= 1,
                _ => cur = plan.update(index, cur),
            }
            sum += cur;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plan_a() {
        let plan: Plan = "A:+,-,=,=".parse().unwrap();
        assert_eq!(103, plan.execute(10));
    }

    #[test]
    fn test_plan_b() {
        let plan: Plan = "B:+,=,-,+".parse().unwrap();
        assert_eq!(116, plan.execute(10));
    }

    #[test]
    fn test_plan_a_part_2() {
        let plan: Plan = "A:+,-,=,=".parse().unwrap();
        let track = Track::init("input/test-track.txt");

        assert_eq!(1290, track.execute_plan(&plan, 10));
    }

    #[test]
    fn test_plan_b_part_2() {
        let plan: Plan = "B:+,=,-,+".parse().unwrap();
        let track = Track::init("input/test-track.txt");
        assert_eq!(3640, track.execute_plan(&plan, 10));
    }
}
