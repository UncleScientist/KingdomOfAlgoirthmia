fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2024_q08_p1.txt");
    let blocks: u64 = lines[0].parse().unwrap();

    let prev_pyramid = (blocks as f64).sqrt() as u64;
    let final_pyramid = prev_pyramid + 1;
    let difference = final_pyramid * final_pyramid - blocks;
    println!("part 1 = {}", difference * (final_pyramid + prev_pyramid));

    let lines = aoclib::read_lines("input/everybody_codes_e2024_q08_p2.txt");
    let priests: u64 = lines[0].parse().unwrap();

    let pyramid = Pyramid {
        priests,
        acolytes: 1111,
        blocks: 20240000,
    };
    println!("part 2 = {}", pyramid.solve());
}

#[derive(Debug)]
struct Pyramid {
    priests: u64,
    acolytes: u64,
    blocks: u64,
}

impl Pyramid {
    fn solve(&self) -> u64 {
        let mut sum = 1;
        let mut prev = 1;
        let mut width = 1;
        while sum < self.blocks {
            let thickness = (prev * self.priests) % self.acolytes;
            width += 2;
            sum += width * thickness;
            prev = thickness;
        }

        width * (sum - self.blocks)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_2() {
        let pyramid = Pyramid {
            priests: 3,
            acolytes: 5,
            blocks: 50,
        };

        assert_eq!(27, pyramid.solve());
    }
}
