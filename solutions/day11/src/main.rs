use std::collections::HashMap;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q11_p1.txt");

    let map = lines
        .iter()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let right = right.split(',').collect();
            (left, right)
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    let mut generation = vec!["A"];
    for _ in 0..4 {
        let mut next_gen = Vec::<&str>::new();
        for g in generation {
            if let Some(entry) = map.get(&g) {
                next_gen.extend(entry);
            }
        }
        generation = next_gen;
    }
    println!("part 1 = {}", generation.len());
}
