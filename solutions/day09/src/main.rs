fn main() {
    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p1.txt");
    let stamps = aoclib::CoinChange::new(&[1, 3, 5, 10]);
    println!(
        "part 1 = {}",
        brightnesses
            .iter()
            .filter_map(|b| stamps.min_coins(*b))
            .sum::<usize>()
    );

    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p2.txt");
    let stamps = aoclib::CoinChange::new(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30]);
    println!(
        "part 2 = {}",
        brightnesses
            .iter()
            .filter_map(|b| stamps.min_coins(*b))
            .sum::<usize>()
    );
}
