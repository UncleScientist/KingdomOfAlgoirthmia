use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q10_p1.txt");
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("part 1 = {}", find_word(&grid, 0, 0));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q10_p2.txt");
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total_power = 0;
    let mut startrow = 0;
    while startrow < grid.len() {
        let mut startcol = 0;
        while startcol < grid[0].len() {
            let word = find_word(&grid, startrow, startcol);
            total_power += word_power(word);
            startcol += 9;
        }
        startrow += 8;
    }
    println!("part 2 = {total_power}");

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q10_p3.txt");
    let mut grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut changed = true;
    while changed {
        changed = false;
        // println!("----- start of loop -----");
        let mut startrow = 0;
        while startrow < grid.len() - 5 {
            let mut startcol = 0;
            while startcol < grid[0].len() - 5 {
                changed |= fill_in(&mut grid, startrow, startcol);
                changed |= deduce(&mut grid, startrow, startcol);
                startcol += 6;
            }
            startrow += 6;
        }

        // for line in &grid { println!("{}", line.iter().collect::<String>()); }
    }
    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }

    let mut total_power = 0;
    let mut startrow = 0;
    while startrow < grid.len() - 5 {
        let mut startcol = 0;
        while startcol < grid[0].len() - 5 {
            total_power += calculate_word(&grid, startrow, startcol);
            startcol += 6;
        }
        startrow += 6;
    }
    println!("part 3 = {total_power}");
}

fn fill_in(grid: &mut [Vec<char>], startrow: usize, startcol: usize) -> bool {
    let mut changed = false;
    for row in 2..6 {
        for col in 2..6 {
            if grid[startrow + row][startcol + col] != '.' {
                continue;
            }

            let row_letters = (0..8)
                .map(|col| grid[startrow + row][startcol + col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let col_letters = (0..8)
                .map(|row| grid[startrow + row][startcol + col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let intersect = row_letters.intersection(&col_letters).collect::<Vec<_>>();
            if intersect.len() == 1 {
                // println!( "fill in at {},{} -> {}", startrow + row, startcol + col, *intersect[0]);
                grid[startrow + row][startcol + col] = *intersect[0];
                changed = true;
            }
        }
    }
    changed
}

fn deduce(grid: &mut [Vec<char>], startrow: usize, startcol: usize) -> bool {
    fn missing(iter: impl Iterator<Item = char>) -> usize {
        iter.filter(|ch| *ch != '.')
            .map(|ch| {
                if ch != '?' {
                    1usize << (ch as u8 - b'A') as usize
                } else {
                    1usize << 27
                }
            })
            .fold(0usize, |result, next| result ^ next)
    }

    let mut changed = false;

    for row in 2..6 {
        for col in 2..6 {
            if grid[startrow + row][startcol + col] != '.' {
                continue;
            }

            let row_letters = missing((0..8).map(|col| grid[startrow + row][startcol + col]));
            let col_letters = missing((0..8).map(|row| grid[startrow + row][startcol + col]));

            if !col_letters.single_bit() || !row_letters.single_bit() {
                continue;
            }

            // print!("deduce for {},{} -> ", startrow + row, startcol + col);
            grid[startrow + row][startcol + col] = if col_letters == 1 << 27 {
                changed = true;

                let ch = (row_letters.trailing_zeros() as u8 + b'A') as char;
                // println!("{ch}");
                for row in 0..8 {
                    if grid[startrow + row][startcol + col] == '?' {
                        // println!( "deduce for ? at {},{} -> {}", startrow + row, startcol + col, ch);
                        grid[startrow + row][startcol + col] = ch;
                        break;
                    }
                }
                ch
            } else {
                changed = true;

                let ch = (col_letters.trailing_zeros() as u8 + b'A') as char;
                // println!("{ch}");
                for col in 0..8 {
                    if grid[startrow + row][startcol + col] == '?' {
                        // println!( "deduce for ? at {},{} -> {}", startrow + row, startcol + col, ch);
                        grid[startrow + row][startcol + col] = ch;
                        break;
                    }
                }
                ch
            }
        }
    }

    changed
}

fn calculate_word(grid: &[Vec<char>], startrow: usize, startcol: usize) -> usize {
    let mut sum = 0;
    let mut index = 1;
    let mut set = 0;
    for row in 2..6 {
        for col in 2..6 {
            let ch = grid[startrow + row][startcol + col];
            if ch == '.' {
                return 0;
            }
            let id = (ch as u8 - b'A') as usize;
            if set & (1 << id) != 0 {
                return 0;
            }
            set |= 1 << id;
            sum += index * ((ch as u8 - b'A') as usize);
            index += 1;
        }
    }

    sum
}

fn find_word(grid: &[Vec<char>], startrow: usize, startcol: usize) -> String {
    let mut answer = String::new();
    for row in 2..6 {
        for col in 2..6 {
            let row_letters = (0..8)
                .map(|col| grid[startrow + row][startcol + col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let col_letters = (0..8)
                .map(|row| grid[startrow + row][startcol + col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let intersect = row_letters.intersection(&col_letters).collect::<Vec<_>>();
            assert!(intersect.len() == 1);
            answer.push(*intersect[0]);
        }
    }
    answer
}

fn word_power(word: impl AsRef<str>) -> usize {
    word.as_ref()
        .chars()
        .enumerate()
        .map(|(index, ch)| (index + 1) * ((ch as u8 - b'A' + 1) as usize))
        .sum()
}

trait SingleBit {
    fn single_bit(self) -> bool;
}

impl SingleBit for usize {
    fn single_bit(self) -> bool {
        self != 0 && ((self & (self - 1)) == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_power() {
        assert_eq!(1851, word_power("PTBVRCZHFLJWGMNS"));
    }

    #[test]
    fn test_part_3_fill() {
        let lines = aoclib::read_lines("input/test-part-3.txt");
        let mut grid = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for line in &grid {
            println!("{line:?}");
        }
        fill_in(&mut grid, 0, 0);
        fill_in(&mut grid, 0, 6);
        println!("-");
        for line in &grid {
            println!("{line:?}");
        }
        deduce(&mut grid, 0, 0);
        deduce(&mut grid, 0, 6);
        for line in &grid {
            println!("{line:?}");
        }
        let word1 = find_word(&grid, 0, 0);
        let word2 = find_word(&grid, 0, 6);
        assert_eq!("LWGVXSHBPJQKNFZM", word1);
        assert_eq!("DQWLXCNHVKJTGFSZ", word2);
    }
}
