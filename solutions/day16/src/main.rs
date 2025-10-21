use std::{collections::HashMap, path::Path};

const PART2_PULLS: usize = 202420242024;

fn main() {
    // let lines = aoclib::read_lines("input/test_1.txt");
    let machine = SlotMachine::read_faces("input/everybody_codes_e2024_q16_p1.txt");
    println!("part 1 = {}", machine.faces_after_n_spins(100));

    // let machine = SlotMachine::read_faces("input/test_1.txt");
    let machine = SlotMachine::read_faces("input/everybody_codes_e2024_q16_p2.txt");
    println!("part 2 = {}", machine.find_part_2(PART2_PULLS));
}

struct SlotMachine {
    spins: Vec<usize>,
    wheels: Vec<Vec<String>>,
}

impl SlotMachine {
    fn read_faces<P: AsRef<Path>>(path: P) -> Self {
        let lines = aoclib::read_lines(path);
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

        Self { spins, wheels }
    }

    fn faces_after_n_spins(&self, spins: usize) -> String {
        (0..self.wheels.len())
            .map(|wheel| {
                self.wheels[wheel][(self.spins[wheel] * spins) % self.wheels[wheel].len()].clone()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn pos_after_pulls(&self, pulls: usize) -> Vec<usize> {
        (0..self.wheels.len())
            .map(|wheel| (self.spins[wheel] * pulls) % self.wheels[wheel].len())
            .collect()
    }

    fn calc_score(&self, pos: &[usize]) -> usize {
        let mut counts = vec![0usize; 256];
        for (idx, p) in pos.iter().enumerate() {
            let mut c = self.wheels[idx][*p].chars();
            counts[c.nth(0).unwrap() as u8 as usize] += 1;
            counts[c.nth(1).unwrap() as u8 as usize] += 1;
        }

        counts.iter().map(|count| (*count).saturating_sub(2)).sum()
    }

    fn find_part_2(&self, spins: usize) -> usize {
        let mut pullscore = HashMap::<Vec<usize>, usize>::new();

        let mut pos = 0;
        let mut running_total = 0;
        loop {
            let positions = self.pos_after_pulls(pos);
            if pullscore.contains_key(&positions) {
                break;
            }
            running_total += self.calc_score(&positions);
            pullscore.insert(positions, running_total);
            pos += 1;
        }

        let full_loops = spins / pos;
        let subtotal = running_total * full_loops;
        let final_position = self.pos_after_pulls(spins - 1);
        let remaining_score = pullscore
            .get(&final_position)
            .unwrap_or_else(|| panic!("missing entry for {final_position:?}"));

        subtotal + remaining_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_score() {
        let machine = SlotMachine::read_faces("input/test_1.txt");
        let positions = machine.pos_after_pulls(3);
        assert_eq!(2, machine.calc_score(&positions));
        let positions = machine.pos_after_pulls(33);
        assert_eq!(4, machine.calc_score(&positions));
    }
}
