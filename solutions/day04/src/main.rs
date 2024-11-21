use std::{iter::Sum, ops::Sub};

fn main() {
    let nails: Vec<i64> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p1.txt");
    let min = nails.iter().min().unwrap();
    println!("part 1 = {}", nails.diffsum(min));

    let nails: Vec<i32> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p2.txt");
    let min = nails.iter().min().unwrap();
    println!("part 2 = {}", nails.diffsum(min));

    let nails: Vec<u128> = aoclib::read_numbers("input/everybody_codes_e2024_q04_p3.txt");
    println!("part 3 = {}", nails.mindiff());
}

trait DiffSum<T> {
    fn diffsum(&self, num: &T) -> T;
}

impl<T: Sum<T> + AbsDiff<T>> DiffSum<T> for Vec<T> {
    fn diffsum(&self, num: &T) -> T {
        self.iter().map(|other| num.abs_diff(*other)).sum()
    }
}

trait MinDiff<T> {
    fn mindiff(&self) -> T;
}

impl<T: Ord + Sum<T> + AbsDiff<T>> MinDiff<T> for Vec<T> {
    fn mindiff(&self) -> T {
        self.iter().map(|nail| self.diffsum(nail)).min().unwrap()
    }
}

trait AbsDiff<T>
where
    Self: Copy + Sub<Output = Self> + PartialOrd,
{
    fn abs_diff(self, other: Self) -> Self {
        if self < other {
            other - self
        } else {
            self - other
        }
    }
}

impl<T> AbsDiff<T> for f32 {}
impl<T> AbsDiff<T> for f64 {}

impl<T> AbsDiff<T> for i8 {}
impl<T> AbsDiff<T> for u8 {}
impl<T> AbsDiff<T> for i16 {}
impl<T> AbsDiff<T> for u16 {}
impl<T> AbsDiff<T> for i32 {}
impl<T> AbsDiff<T> for u32 {}
impl<T> AbsDiff<T> for i64 {}
impl<T> AbsDiff<T> for u64 {}
impl<T> AbsDiff<T> for i128 {}
impl<T> AbsDiff<T> for u128 {}
impl<T> AbsDiff<T> for isize {}
impl<T> AbsDiff<T> for usize {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(10, vec![3, 4, 7, 8].diffsum(&3));
    }

    #[test]
    fn test_part3() {
        assert_eq!(8, vec![2, 4, 5, 6, 8].mindiff());
    }
}
