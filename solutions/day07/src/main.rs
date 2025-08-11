use std::str::FromStr;

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
}

#[derive(Debug)]
enum Action {
    Increase,
    Decrease,
    Same,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Increase),
            "-" => Ok(Self::Decrease),
            "=" => Ok(Self::Same),
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
}
