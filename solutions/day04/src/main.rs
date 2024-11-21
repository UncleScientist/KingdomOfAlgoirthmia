fn main() {
    let nails: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p1.txt");
    println!("part 1 = {}", parts_1and2(&nails));

    let nails: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p2.txt");
    println!("part 2 = {}", parts_1and2(&nails));

    let nails: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p3.txt");
    println!("part 3 = {}", part3(&nails));
}

fn parts_1and2(nails: &[usize]) -> usize {
    let min = nails.iter().min().unwrap();
    nails.iter().map(|nail| *nail - min).sum()
}

fn part3(nails: &[usize]) -> usize {
    nails
        .iter()
        .map(|nail| nails.iter().map(|other| nail.abs_diff(*other)).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(10, parts_1and2(&[3, 4, 7, 8]));
    }

    #[test]
    fn test_part3() {
        assert_eq!(8, part3(&[2, 4, 5, 6, 8]));
    }
}
