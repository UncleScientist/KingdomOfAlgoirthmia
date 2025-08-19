fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q08_p1.txt");
    let blocks: u64 = lines[0].parse().unwrap();

    let prev_pyramid = (blocks as f64).sqrt() as u64;
    let final_pyramid = prev_pyramid + 1;
    let difference = final_pyramid * final_pyramid - blocks;
    println!("part 1 = {}", difference * (final_pyramid + prev_pyramid));
}
