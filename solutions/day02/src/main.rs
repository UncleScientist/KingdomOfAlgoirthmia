fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q02_p1.txt");
    let (_, wordlist) = lines[0].split_once(':').unwrap();
    let words = wordlist.split(',').collect::<Vec<_>>();

    println!("part 1 = {}", part1(&words, &lines[1]));
}

fn part1(words: &[&str], line: &str) -> usize {
    words
        .iter()
        .map(|word| {
            (0..line.len() - word.len())
                .filter(|idx| line[*idx..].starts_with(word))
                .count()
        })
        .sum()
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
}
