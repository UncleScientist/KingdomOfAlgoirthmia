fn main() {
    let brightnesses: Vec<usize> = aoclib::read_numbers("input/everybody_codes_e2024_q09_p1.txt");
    let coins = aoclib::CoinChange::new(&[1, 3, 5, 10]);
    println!(
        "part 1 = {}",
        brightnesses
            .iter()
            .filter_map(|b| coins.min_coins(*b))
            .sum::<usize>()
    );
}
