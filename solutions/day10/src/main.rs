use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q10_p1.txt");
    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut answer = String::new();
    for row in 2..6 {
        for col in 2..6 {
            let row_letters = (0..8)
                .map(|col| grid[row][col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let col_letters = (0..8)
                .map(|row| grid[row][col])
                .filter(|ch| *ch != '.')
                .collect::<HashSet<char>>();
            let intersect = row_letters.intersection(&col_letters).collect::<Vec<_>>();
            assert!(intersect.len() == 1);
            answer.push(*intersect[0]);
        }
    }
    println!("part 1 = {answer}");
}
