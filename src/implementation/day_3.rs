use crate::Solution;

pub struct DayThreeSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DayThreeSolution {
    const DAY: u8 = 3;

    fn new() -> Self {
        DayThreeSolution {
            data: parse_input(&Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u32 {
        // Sum two batteries in each pack
        let mut sum = 0;
        for pack in &self.data {
            let mut first_digit = b'0';
            let mut last_digit = b'0';
            let mut first_digit_pos = 0;
            let mut i = 0;
            while i < pack.len() {
                if first_digit == b'9' {
                    if last_digit == b'9' {
                        break;
                    }
                    if pack[i] > last_digit {
                        last_digit = pack[i];
                    }
                }
                if pack[i] > first_digit && i != pack.len() - 1 {
                    first_digit = pack[i];
                    first_digit_pos = i;
                    last_digit = b'0';
                    i += 1;
                    continue;
                }
                if pack[i] > last_digit && i > first_digit_pos {
                    last_digit = pack[i];
                }
                i += 1;
            }
            sum += (first_digit as char).to_digit(10).unwrap() * 10
                + (last_digit as char).to_digit(10).unwrap();
        }
        sum
    }

    fn part_two(&self) -> u64 {
        // Sum twelve batteries in each pack
        let mut sum = 0;
        let mut digits_vec: Vec<(u8, usize)> = (0..12).map(|_| (0, 0)).collect();
        const CAPACITY: usize = 12;

        for pack in &self.data {
            for vec_i in 0..digits_vec.len() {
                let l = if vec_i == 0 {
                    0
                } else {
                    digits_vec[vec_i - 1].1 + 1
                };
                for i in l..(pack.len() - (CAPACITY - (vec_i + 1))) {
                    if pack[i] > digits_vec[vec_i].0 {
                        digits_vec[vec_i] = (pack[i], i);
                        if pack[i] == b'9' {
                            break;
                        }
                    }
                }
            }
            let mut pack_sum: u64 = 0;
            for i in 0..digits_vec.len() {
                pack_sum += (digits_vec[i].0 as char).to_digit(10).unwrap() as u64
                    * 10_u64.pow((digits_vec.len() - 1 - i) as u32);
            }
            sum += pack_sum;
            digits_vec.fill((b'0', 0));
        }
        sum
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<u8>> {
    input.iter().map(|s| s.as_bytes().to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_3.txt")
            .map(|data| {
                data.lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap();

        let day_three = DayThreeSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_three.part_one();

        assert_eq!(357, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_3.txt")
            .map(|data| {
                data.lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap();

        let day_three = DayThreeSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_three.part_two();

        assert_eq!(3121910778619, sol);
    }
}
