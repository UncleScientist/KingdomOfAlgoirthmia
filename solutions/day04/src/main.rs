fn main() {
    let nails: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p1.txt");
    println!("part 1 = {}", part1(&nails));
}

fn part1(nails: &[usize]) -> usize {
    let min = nails.iter().min().unwrap();
    nails.iter().map(|nail| *nail - min).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(10, part1(&[3, 4, 7, 8]));
    }
}
