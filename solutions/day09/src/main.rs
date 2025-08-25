use aoclib::CoinChange;

fn main() {
    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p1.txt");
    let mut stamps = CoinChange::new(&[1, 3, 5, 10]);
    println!(
        "part 1 = {}",
        brightnesses
            .iter()
            .filter_map(|b| stamps.min_coins(*b))
            .sum::<usize>()
    );

    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p2.txt");
    let mut stamps = CoinChange::new(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30]);
    println!(
        "part 2 = {}",
        brightnesses
            .iter()
            .filter_map(|b| stamps.min_coins(*b))
            .sum::<usize>()
    );

    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p3.txt");
    let mut stamps = CoinChange::new(&[
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ]);
    println!(
        "part 3 = {}",
        brightnesses
            .into_iter()
            .map(|b| find_best_split(&mut stamps, b))
            .sum::<usize>()
    );
}

fn find_best_split(cc: &mut CoinChange<usize>, value: usize) -> usize {
    let mut best = usize::MAX;
    let mut lower = value / 2;
    let mut upper = value - lower;
    while (upper - lower) <= 100 {
        if let Some(u) = cc.min_coins(upper)
            && let Some(l) = cc.min_coins(lower)
        {
            best = best.min(u + l);
        }
        lower -= 1;
        upper = value - lower;
    }
    best
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_value() {
        let mut stamps = CoinChange::new(&[
            1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
        ]);
        assert_eq!(775 + 775, find_best_split(&mut stamps, 156488));
    }
}
