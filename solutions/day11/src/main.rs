use std::collections::HashMap;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p1.txt");
    let termites = Termites::new(&lines);
    println!("part 1 = {}", termites.count("A", 4));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p2.txt");
    let termites = Termites::new(&lines);
    println!("part 2 = {}", termites.count("Z", 10));
}

struct Termites<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Termites<'a> {
    fn new(lines: &'a [String]) -> Self {
        Self {
            map: lines
                .iter()
                .map(|line| {
                    let (left, right) = line.split_once(':').unwrap();
                    let right = right.split(',').collect();
                    (left, right)
                })
                .collect(),
        }
    }

    fn count(&self, start: &str, generations: usize) -> usize {
        let mut generation = vec![start];
        for _ in 0..generations {
            let mut next_gen = Vec::<&str>::new();
            for g in generation {
                if let Some(entry) = self.map.get(&g) {
                    next_gen.extend(entry);
                }
            }
            generation = next_gen;
        }
        generation.len()
    }
}
