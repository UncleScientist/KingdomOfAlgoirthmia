use std::collections::HashMap;

fn main() {
    let data: Vec<Vec<i64>> = aoclib::numbers("input/everybody_codes_e2024_q05_p1.txt", ' ');
    let mut numbers = transpose(&data);
    println!("part 1 = {}", part1(&mut numbers));

    let data: Vec<Vec<i64>> = aoclib::numbers("input/everybody_codes_e2024_q05_p2.txt", ' ');
    let mut numbers = transpose(&data);
    println!("part 2 = {}", part2(&mut numbers));
}

fn part1(numbers: &mut [Vec<i64>]) -> String {
    let mut col = 0;
    for _ in 0..10 {
        let dancer = numbers[col].remove(0);
        col = (col + 1) % numbers.len();
        dance(dancer, &mut numbers[col]);
    }

    numbers.iter().fold(String::new(), |mut output, col| {
        output.push_str(&format!("{}", col[0]));
        output
    })
}

fn part2(numbers: &mut [Vec<i64>]) -> String {
    let mut shouts = HashMap::<String, usize>::new();
    let mut col = 0;
    for round in 1.. {
        let dancer = numbers[col].remove(0);
        col = (col + 1) % numbers.len();
        dance(dancer, &mut numbers[col]);
        let shout = numbers.iter().fold(String::new(), |mut output, col| {
            output.push_str(&format!("{}", col[0]));
            output
        });
        let entry = shouts.entry(shout.clone()).or_default();
        *entry += 1;
        if *entry == 2024 {
            let shout = shout.parse::<usize>().unwrap();
            return format!("{}", shout * round);
        }
    }
    "ran out of numbers in usize".to_string()
}

fn transpose(data: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut numbers = vec![Vec::new(); 4];

    for line in data {
        for (idx, num) in line.iter().enumerate() {
            numbers[idx].push(*num);
        }
    }
    numbers
}

//   9  8  7  6  5
//   v  v  v  v  v
//  [ 3  4  5  2  ]
//   ^  ^  ^  ^
//   1   2  3  4

fn dance(dancer: i64, line: &mut Vec<i64>) {
    let size = 2 * line.len();
    let pos = ((dancer - 1) as usize) % size;
    if pos < line.len() {
        line.insert(pos, dancer);
    } else {
        let pos = size - pos;
        line.insert(pos, dancer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn part1_data() -> Vec<Vec<i64>> {
        vec![
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 2],
            vec![4, 5, 2, 3],
            vec![5, 2, 3, 4],
        ]
    }

    #[test]
    fn test_transpose() {
        let data = part1_data();
        let numbers = transpose(&data);
        assert_eq!(vec![2, 3, 4, 5], numbers[0]);
        assert_eq!(vec![5, 2, 3, 4], numbers[3]);
    }

    #[test]
    fn test_forward_insert() {
        let data = part1_data();
        let mut numbers = transpose(&data);
        dance(2, &mut numbers[1]);
        assert_eq!(vec![3, 2, 4, 5, 2], numbers[1]);
    }

    #[test]
    fn test_reverse_insert() {
        let data = part1_data();
        let mut numbers = transpose(&data);
        dance(6, &mut numbers[1]);
        assert_eq!(vec![3, 4, 5, 6, 2], numbers[1]);
    }

    #[test]
    fn test_end_insert() {
        let data = part1_data();
        let mut numbers = transpose(&data);
        dance(5, &mut numbers[1]);
        assert_eq!(vec![3, 4, 5, 2, 5], numbers[1]);
    }

    #[test]
    fn test_part_2() {
        let data = vec![vec![2, 3, 4, 5], vec![6, 7, 8, 9]];
        let mut numbers = transpose(&data);
        assert_eq!("50877075".to_string(), part2(&mut numbers));
    }
}
