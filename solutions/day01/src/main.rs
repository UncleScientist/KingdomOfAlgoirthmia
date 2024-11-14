fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q01_p1.txt");
    println!("part 1 = {}", part1(&lines[0]));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q01_p2.txt");
    println!("part 2 = {}", part2(&lines[0]));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q01_p3.txt");
    println!("part 3 = {}", part3(&lines[0]));
}

fn damage(ch: char) -> usize {
    match ch {
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn part1(s: &str) -> usize {
    s.chars().map(damage).sum()
}

fn part2(s: &str) -> usize {
    let v = s.chars().collect::<Vec<_>>();
    v.as_slice()
        .chunks(2)
        .map(|pair| {
            damage(pair[0])
                + damage(pair[1])
                + if pair[0] != 'x' && pair[1] != 'x' {
                    2
                } else {
                    0
                }
        })
        .sum()
}

fn part3(s: &str) -> usize {
    let v = s.chars().collect::<Vec<_>>();
    v.as_slice()
        .chunks(3)
        .map(|triple| {
            let creatures = triple.iter().filter(|x| **x != 'x').count();
            triple.iter().map(|ch| damage(*ch)).sum::<usize>()
                + match creatures {
                    2 => 2,
                    3 => 6,
                    _ => 0,
                }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, part1("ABBAC"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(28, part2("AxBCDDCAxD"));
    }

    #[test]
    fn test_part3() {
        assert_eq!(30, part3("xBxAAABCDxCC"));
    }
}
