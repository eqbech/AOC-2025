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
            for i in 0..pack.len() - 1 {
                if pack[i] > first_digit {
                    first_digit = pack[i];
                    first_digit_pos = i;
                    if pack[i] == b'9' {
                        break;
                    }
                }
            }
            for j in (first_digit_pos + 1)..pack.len() {
                if pack[j] > last_digit {
                    last_digit = pack[j];
                    if pack[j] == b'9' {
                        break;
                    }
                }
            }
            sum += [first_digit as char, last_digit as char].iter().collect::<String>().parse::<u32>()
                .unwrap();
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
            let pack_sum: String = digits_vec.iter().map(|(d, _)| *d as char).collect();
            sum += pack_sum.parse::<u64>().unwrap();
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

        let day_three = DayThreeSolution { data: parse_input(&test_vec) };
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

        let day_three = DayThreeSolution { data: parse_input(&test_vec) };
        let sol = day_three.part_two();

        assert_eq!(3121910778619, sol);
    }
}
