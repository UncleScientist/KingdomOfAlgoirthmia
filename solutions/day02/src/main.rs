use std::collections::{HashMap, HashSet};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q02_p1.txt");
    let (_, wordlist) = lines[0].split_once(':').unwrap();
    let words = wordlist.split(',').collect::<Vec<_>>();

    println!("part 1 = {}", part1(&words, &lines[1]));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q02_p2.txt");
    let (_, wordlist) = lines[0].split_once(':').unwrap();
    let words = wordlist.split(',').collect::<Vec<_>>();

    println!(
        "part 2 = {}",
        lines[1..]
            .iter()
            .fold(0, |num, line| num + part2(&words, line))
    );

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q02_p3.txt");
    let (_, wordlist) = lines[0].split_once(':').unwrap();
    let words = wordlist.split(',').collect::<Vec<_>>();
    println!("part 3 = {}", part3(&words, &lines[1..]));
}

fn part1(words: &[&str], line: &str) -> usize {
    words
        .iter()
        .map(|word| {
            (0..line.len() - word.len() + 1)
                .filter(|idx| line[*idx..].starts_with(word))
                .count()
        })
        .sum()
}

fn part2(words: &[&str], line: &str) -> usize {
    let mut runepos = HashSet::new();
    let rev = line.chars().rev().collect::<String>();

    for word in words {
        for idx in 0..line.len() - word.len() + 1 {
            if line[idx..].starts_with(word) {
                for pos in idx..idx + word.len() {
                    runepos.insert(pos);
                }
            }
            if rev[idx..].starts_with(word) {
                for pos in idx..idx + word.len() {
                    runepos.insert(line.len() - pos - 1);
                }
            }
        }
    }

    runepos.len()
}

fn part3<S: AsRef<str>>(words: &[&str], lines: &[S]) -> usize {
    let mut matrix = HashMap::<(i64, i64), char>::new();
    let rows = lines.len() as i64;
    let cols = lines[0].as_ref().len() as i64;

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.as_ref().chars().enumerate() {
            matrix.insert((row as i64, col as i64), ch);
        }
    }

    let mut runepos = HashSet::new();

    for word in words {
        let chars = word.chars().collect::<Vec<_>>();
        for r in 0..rows {
            for c in 0..cols {
                'next_dir: for dir in [(-1_i64, 0_i64), (1, 0), (0, -1), (0, 1)] {
                    let mut rloc = r;
                    let mut cloc = c;

                    for ch in &chars {
                        if rloc < 0 || rloc >= rows {
                            continue 'next_dir;
                        }

                        if *ch != *matrix.get(&(rloc, cloc)).unwrap() {
                            continue 'next_dir;
                        }
                        rloc += dir.0;
                        cloc += dir.1;
                        if cloc < 0 {
                            cloc = cols - 1;
                        } else if cloc >= cols {
                            cloc = 0;
                        }
                    }
                    rloc = r;
                    cloc = c;

                    for _ in 0..word.len() {
                        runepos.insert((rloc, cloc));
                        rloc += dir.0;
                        cloc += dir.1;
                        if cloc < 0 {
                            cloc = cols - 1;
                        } else if cloc >= cols {
                            cloc = 0;
                        }
                    }
                }
            }
        }
    }
    runepos.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let words = ["THE", "OWE", "MES", "ROD", "HER"];
        assert_eq!(
            3,
            part1(&words, "THE FLAME SHIELDED THE HEART OF THE KINGS")
        );
        assert_eq!(2, part1(&words, "POWE PO WER P OWE R"));
        assert_eq!(3, part1(&words, "THERE IS THE END"));
    }

    #[test]
    fn test_part2() {
        let words = ["THE", "OWE", "MES", "ROD", "HER"];
        assert_eq!(
            15,
            part2(&words, "AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE")
        );
        assert_eq!(
            9,
            part2(&words, "THE FLAME SHIELDED THE HEART OF THE KINGS")
        );
        assert_eq!(6, part2(&words, "POWE PO WER P OWE R"));
        assert_eq!(7, part2(&words, "THERE IS THE END"));
    }

    #[test]
    fn test_overlapping_backwards() {
        let words = ["THE", "OWE", "MES", "ROD", "HER"];
        assert_eq!(8, part2(&words, "THIS IS SEMAPHORE - THEWO"));
    }

    #[test]
    fn test_multiple() {
        let words = ["THE", "OWE", "MES", "ROD", "HER"];
        assert_eq!(12, part2(&words, "MESMESMESMES"));
    }

    #[test]
    fn test_part3() {
        let words = ["THE", "OWE", "MES", "ROD", "RODEO"];
        let lines = ["HELWORLT", "ENIGWDXL", "TRODEOAL"];

        assert_eq!(10, part3(&words, &lines));
    }
}
