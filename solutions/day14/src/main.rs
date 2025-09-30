use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

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
    let schedule = Step::schedule(&lines);
    println!("part 2 = {}", Garden::grow(&schedule).segments.len());

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q14_p3.txt");
    let schedule: Vec<Vec<Step>> = Step::schedule(&lines);
    let garden = Garden::grow(&schedule);

    println!("part 3 = {}", garden.min_murkiness());
}

struct Garden {
    segments: HashSet<(i64, i64, i64)>,
    leaves: Vec<(i64, i64, i64)>,
}

impl Garden {
    fn grow(schedule: &[Vec<Step>]) -> Self {
        let mut segments = HashSet::new();
        let mut leaves = Vec::new();
        for tree in schedule {
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
                for _ in 0..*amount {
                    pos = (pos.0 + delta.0, pos.1 + delta.1, pos.2 + delta.2);
                    segments.insert(pos);
                }
            }
            leaves.push(pos);
        }
        Self { segments, leaves }
    }

    fn trunk(&self) -> Vec<&(i64, i64, i64)> {
        self.segments
            .iter()
            .filter(|seg| seg.1 == 0 && seg.2 == 0)
            .collect()
    }

    fn min_murkiness(&self) -> i64 {
        let trunk = self.trunk();
        let mut min_murkiness = i64::MAX;
        for trunk_seg in trunk {
            let mut queue = VecDeque::from([(0, *trunk_seg)]);
            let mut visited = HashSet::new();
            let mut murkiness = 0;
            let mut remaining: HashSet<_> = self.leaves.iter().collect();
            while let Some((dist, pos)) = queue.pop_front() {
                if remaining.contains(&pos) {
                    murkiness += dist;
                    remaining.remove(&pos);
                    if remaining.is_empty() {
                        break;
                    }
                }
                if visited.insert(pos) {
                    for dir in [
                        (0, 0, 1),
                        (0, 0, -1),
                        (0, 1, 0),
                        (0, -1, 0),
                        (1, 0, 0),
                        (-1, 0, 0),
                    ] {
                        let next = (pos.0 + dir.0, pos.1 + dir.1, pos.2 + dir.2);
                        if self.segments.contains(&next) {
                            queue.push_back((dist + 1, next));
                        }
                    }
                }
            }
            min_murkiness = min_murkiness.min(murkiness);
        }

        min_murkiness
    }
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

impl Step {
    fn schedule<S: AsRef<str>>(lines: &[S]) -> Vec<Vec<Step>> {
        lines
            .iter()
            .map(|line| {
                line.as_ref()
                    .split(',')
                    .map(|step| step.parse().unwrap())
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() {
        let lines = ["U5,R3,D2,L5,U4,R5,D2", "U6,L1,D2,R3,U2,L1"];
        let schedule = Step::schedule(&lines);
        assert_eq!(32, Garden::grow(&schedule).segments.len());
    }

    #[test]
    fn test_part_3_simple() {
        let lines = ["U5,R3,D2,L5,U4,R5,D2", "U6,L1,D2,R3,U2,L1"];
        let schedule = Step::schedule(&lines);
        assert_eq!(5, Garden::grow(&schedule).min_murkiness());
    }

    #[test]
    fn test_part_3_difficult() {
        let lines = [
            "U20,L1,B1,L2,B1,R2,L1,F1,U1",
            "U10,F1,B1,R1,L1,B1,L1,F1,R2,U1",
            "U30,L2,F1,R1,B1,R1,F2,U1,F1",
            "U25,R1,L2,B1,U1,R2,F1,L2",
            "U16,L1,B1,L1,B3,L1,B1,F1",
        ];
        let schedule = Step::schedule(&lines);
        assert_eq!(46, Garden::grow(&schedule).min_murkiness());
    }
}
