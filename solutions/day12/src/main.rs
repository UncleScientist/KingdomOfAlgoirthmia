use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q12_p1.txt");
    // let lines = aoclib::read_lines("input/test-1.txt");

    let mut targets: HashSet<(i64, i64)> = HashSet::new();
    let mut catapults = Vec::new();
    let mut ground = 0;
    let mut max_x = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'A' | 'B' | 'C' => {
                    catapults.push(((ch as u8 - b'A' + 1) as i64, row as i64, col as i64));
                }
                'T' => {
                    targets.insert((row as i64, col as i64));
                    max_x = max_x.max(col as i64 + 1);
                }
                '.' => {}
                '=' => {
                    ground = row as i64;
                }
                _ => panic!("invalid char '{ch}'"),
            }
        }
    }

    let mut result = 0;
    while !targets.is_empty() {
        for (id, start_row, start_col) in &catapults {
            let mut power = 1;
            while power * 2 + start_col < max_x {
                let mut cur_row = start_row - power + 1;
                let mut cur_col = power * 2 + start_col + 1;
                while cur_row < ground {
                    if targets.contains(&(cur_row, cur_col))
                        && !targets.contains(&(cur_row - 1, cur_col))
                    {
                        targets.remove(&(cur_row, cur_col));
                        result += *id * power;
                        break;
                    }
                    cur_row += 1;
                    cur_col += 1;
                }

                power += 1;
            }
        }
    }
    println!("part 1 = {result}");
}
