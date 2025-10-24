use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q18_p1.txt");
    // let lines = aoclib::read_lines("input/test_1.txt");

    let mut maze = HashSet::<(i64, i64)>::new();
    let mut palms = HashSet::<(i64, i64)>::new();
    let mut start = None;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let (row, col) = (row as i64, col as i64);
            match ch {
                '#' => {}
                '.' => {
                    maze.insert((row, col));
                    if col == 0 {
                        start = Some((row, col));
                    }
                }
                'P' => {
                    maze.insert((row, col));
                    palms.insert((row, col));
                }
                _ => {
                    panic!("invalid char '{ch}'");
                }
            }
        }
    }

    let Some(start) = start else {
        panic!("starting location not found");
    };

    let mut queue = HashSet::from([start]);
    let mut visited = HashSet::new();
    let mut time = 0;
    while !palms.is_empty() {
        let mut next_step = HashSet::new();
        for pos in queue {
            if visited.insert(pos) {
                palms.remove(&pos);
                for delta in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let newpos = (pos.0 + delta.0, pos.1 + delta.1);
                    if maze.contains(&newpos) {
                        next_step.insert(newpos);
                    }
                }
            }
        }
        queue = next_step;
        if !palms.is_empty() {
            time += 1;
        }
    }

    println!("part 1 = {time}");
}
