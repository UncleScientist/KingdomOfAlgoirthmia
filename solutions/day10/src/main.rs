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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_power() {
        assert_eq!(1851, word_power("PTBVRCZHFLJWGMNS"));
    }
}
