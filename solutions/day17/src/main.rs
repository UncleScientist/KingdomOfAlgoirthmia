use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q17_p1.txt");
    let (star_count, edge_list) = generate_edges(&lines, usize::MAX);
    println!("part 1 = {}", star_count + prim(&edge_list).1);

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q17_p2.txt");
    let (star_count, edge_list) = generate_edges(&lines, usize::MAX);
    println!("part 2 = {}", star_count + prim(&edge_list).1);

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q17_p3.txt");
    let (_, mut edge_list) = generate_edges(&lines, 6);

    let mut constellations = BinaryHeap::new();
    while !edge_list.is_empty() {
        let (found, distance) = prim(&edge_list);
        constellations.push(distance + found.len());
        edge_list.retain(|(_, i, j)| !found.contains(i) && !found.contains(j));
    }

    let part3 = constellations.pop().unwrap()
        * constellations.pop().unwrap()
        * constellations.pop().unwrap();
    println!("part 3 = {part3}",);
}

fn generate_edges(lines: &[String], limit: usize) -> (usize, Vec<(usize, usize, usize)>) {
    let mut stars: Vec<(usize, usize)> = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                stars.push((row, col));
            }
        }
    }

    let mut edge_list = Vec::new();
    for i in 0..stars.len() - 1 {
        for j in i + 1..stars.len() {
            let dist = stars[i].0.abs_diff(stars[j].0) + stars[i].1.abs_diff(stars[j].1);
            if dist < limit {
                edge_list.push((dist, i, j));
            }
        }
    }

    (stars.len(), edge_list)
}

// vec: (dist, i, j)
fn prim(edge_list: &[(usize, usize, usize)]) -> (HashSet<usize>, usize) {
    let start = edge_list[0].1;

    let mut queue = edge_list
        .iter()
        .filter(|(_, vertex, _)| *vertex == start)
        .copied()
        .map(Reverse)
        .collect::<BinaryHeap<_>>();

    let mut visited = HashSet::from([start]);

    let mut total = 0;
    while let Some(Reverse((cost, _e1, e2))) = queue.pop() {
        if visited.insert(e2) {
            total += cost;
            for (new_cost, new_e1, new_e2) in edge_list {
                if e2 == *new_e1 && !visited.contains(new_e2) {
                    queue.push(Reverse((*new_cost, e2, *new_e2)));
                } else if e2 == *new_e2 && !visited.contains(new_e1) {
                    queue.push(Reverse((*new_cost, e2, *new_e1)));
                }
            }
        }
    }

    (visited, total)
}
