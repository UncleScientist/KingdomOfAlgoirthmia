use std::{
    collections::{HashMap, HashSet},
    path::Path,
    str::FromStr,
};

use aoclib::Permutations;

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

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q07_p3.txt");
    let rival_plan: Plan = lines[0].parse().unwrap();
    let track = Track::init("input/track-part-3.txt");
    let rival_score = track.execute_plan(&rival_plan, 2024);

    let options = vec!["+", "+", "+", "+", "+", "-", "-", "-", "=", "=", "="];
    let mut seen = HashSet::new();
    let mut better = 0;
    for permutation in options.permutations() {
        if seen.insert(permutation.clone()) {
            let plan_string = format!("P:{}", permutation.join(","));
            let plan: Plan = plan_string.parse().unwrap();
            better += (track.execute_plan(&plan, 2024) > rival_score) as usize;
        }
    }
    println!("part 3 = {better}");
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

        let mut grid: HashMap<(i64, i64), Action> = HashMap::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    grid.insert((row as i64, col as i64), ch.into());
                }
            }
        }

        let dirs = [
            [(-1i64, 0i64), (0, 1), (1, 0)], /* facing east */
            [(0, 1), (1, 0), (0, -1)],       /* facing south */
            [(1, 0), (0, -1), (-1, 0)],      /* facing west */
            [(0, -1), (-1, 0), (0, 1)],      /* facing north */
        ];

        assert_eq!(grid.get(&(0, 0)), Some(Action::StartEnd).as_ref());
        track.push(Action::StartEnd);

        let mut pos = (0, 1);
        let mut facing = 0;
        while pos != (0, 0) {
            track.push(*grid.get(&pos).unwrap());
            for (index, dir) in dirs[facing].iter().enumerate() {
                let newpos = (pos.0 + dir.0, pos.1 + dir.1);
                if grid.contains_key(&newpos) {
                    pos = newpos;
                    facing = (facing + index + (dirs.len() - 1)) % dirs.len();
                    break;
                }
            }
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
    fn test_track() {
        let track = Track::init("input/test-track.txt");
        use Action::*;
        assert_eq!(
            vec![
                StartEnd, Increase, Equal, Equal, Equal, Increase, Increase, Decrease, Equal,
                Increase, Equal, Decrease
            ],
            track.track
        );
    }

    #[test]
    fn test_plan_b_part_2() {
        let plan: Plan = "B:+,=,-,+".parse().unwrap();
        let track = Track::init("input/test-track.txt");
        assert_eq!(3640, track.execute_plan(&plan, 10));
    }
}
