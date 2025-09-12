use std::collections::HashMap;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p1.txt");
    let termites = Termites::new(&lines);
    println!("part 1 = {}", termites.count("A", 4));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p2.txt");
    let termites = Termites::new(&lines);
    println!("part 2 = {}", termites.count("Z", 10));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p3.txt");
    let termites = Termites::new(&lines);
    let mut min = usize::MAX;
    let mut max = 0;
    let mut cache = Cache::default();
    for key in termites.keys() {
        let count = cache.count(&termites, key, 20);
        min = min.min(count);
        max = max.max(count);
    }
    println!("part 3 = {}", max - min);
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
        for i in 0..generations {
            if i > 10 {
                println!("{i}");
            }
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

    fn keys(&self) -> Vec<&str> {
        self.map.keys().copied().collect()
    }
}

#[derive(Debug, Default)]
struct Cache<'a> {
    cache: HashMap<(&'a str, usize), usize>,
}

impl<'a> Cache<'a> {
    fn count(&mut self, termites: &'a Termites, start: &'a str, generations: usize) -> usize {
        if let Some(count) = self.cache.get(&(start, generations)) {
            return *count;
        }
        if generations == 1 {
            return termites.map.get(start).unwrap().len();
        }
        let result = termites
            .map
            .get(start)
            .unwrap()
            .iter()
            .map(|termite| self.count(termites, termite, generations - 1))
            .sum();
        self.cache.insert((start, generations), result);
        result
    }
}
