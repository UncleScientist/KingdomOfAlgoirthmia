fn main() {
    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q16_p1.txt");
    let spins = lines[0]
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let columns = (lines[1].len() + 1) / 4;
    let mut wheels: Vec<Vec<String>> = vec![Vec::new(); columns];

    for line in &lines[1..] {
        for (pos, wheel) in wheels.iter_mut().enumerate() {
            let start = pos * 4;
            if start >= line.len() {
                break;
            }

            let s = line[start..start + 3].to_string();
            if s != "   " {
                wheel.push(s);
            }
        }
    }

    let result = (0..columns)
        .map(|wheel| wheels[wheel][(spins[wheel] * 100) % wheels[wheel].len()].clone())
        .collect::<Vec<_>>();
    println!("part 1 = {}", result.join(" "));
}
