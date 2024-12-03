fn main() {
    let data: Vec<Vec<i64>> = aoclib::numbers("input/everybody_codes_e2024_q05_p1.txt", ' ');
    let mut numbers = transpose(&data);
    println!("part 1 = {}", part1(&mut numbers));
}

fn part1(numbers: &mut [Vec<i64>]) -> String {
    let mut col = 0;
    for _ in 0..10 {
        let dancer = numbers[col].remove(0);
        col = (col + 1) % numbers.len();
        dance(dancer, &mut numbers[col]);
    }

    numbers.iter().map(|col| format!("{}", col[0])).collect()
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
    let pos = (dancer - 1) as usize;
    if pos < line.len() {
        line.insert(pos, dancer);
    } else {
        let pos = line.len() - (pos - line.len());
        line.insert(pos, dancer);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn data() -> Vec<Vec<i64>> {
        vec![
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 2],
            vec![4, 5, 2, 3],
            vec![5, 2, 3, 4],
        ]
    }

    #[test]
    fn test_transpose() {
        let data = data();
        let numbers = transpose(&data);
        assert_eq!(vec![2, 3, 4, 5], numbers[0]);
        assert_eq!(vec![5, 2, 3, 4], numbers[3]);
    }

    #[test]
    fn test_forward_insert() {
        let data = data();
        let mut numbers = transpose(&data);
        dance(2, &mut numbers[1]);
        assert_eq!(vec![3, 2, 4, 5, 2], numbers[1]);
    }

    #[test]
    fn test_reverse_insert() {
        let data = data();
        let mut numbers = transpose(&data);
        dance(6, &mut numbers[1]);
        assert_eq!(vec![3, 4, 5, 6, 2], numbers[1]);
    }

    #[test]
    fn test_end_insert() {
        let data = data();
        let mut numbers = transpose(&data);
        dance(5, &mut numbers[1]);
        assert_eq!(vec![3, 4, 5, 2, 5], numbers[1]);
    }
}
