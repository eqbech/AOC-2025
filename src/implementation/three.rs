use crate::Solution;

pub struct DayThreeSolution {
    data: Vec<String>,
}

impl Solution for DayThreeSolution {
    const DAY: u8 = 3;

    fn new() -> Self {
        DayThreeSolution {
            data: Self::read_data_to_vec().unwrap(),
        }
    }

    fn part_one(&self) -> u32 {
        // Sum two batteries in each pack
        let mut sum = 0;
        for pack in &self.data {
            let pack_chars: Vec<char> = pack.chars().collect();
            let mut first_digit = 0;
            let mut last_digit = 0;
            let mut first_digit_pos = 0;
            for i in 0..pack_chars.len() - 1 {
                if pack_chars[i].to_digit(10).unwrap() > first_digit {
                    first_digit = pack_chars[i].to_digit(10).unwrap();
                    first_digit_pos = i;
                    if pack_chars[i] == '9' {
                        break;
                    }
                }
            }
            for j in (first_digit_pos + 1)..pack_chars.len() {
                if pack_chars[j].to_digit(10).unwrap() > last_digit {
                    last_digit = pack_chars[j].to_digit(10).unwrap();
                    if pack_chars[j] == '9' {
                        break;
                    }
                }
            }
            sum += format!("{}{}", first_digit, last_digit)
                .parse::<u32>()
                .unwrap();
        }
        sum
    }

    fn part_two(&self) -> u64 {
        // Sum twelve batteries in each pack
        let mut sum = 0;
        let mut digits_vec: Vec<(u64, usize)> = (0..12).map(|_| (0, 0)).collect();
        const CAPACITY: usize = 12;

        for pack in &self.data {
            let pack_chars: Vec<char> = pack.chars().collect();
            for vec_i in 0..digits_vec.len() {
                let l = if vec_i == 0 {
                    0
                } else {
                    digits_vec[vec_i - 1].1 + 1
                };
                for i in l..(pack_chars.len() - (CAPACITY - (vec_i + 1))) {
                    let digit = pack_chars[i].to_digit(10).unwrap() as u64;
                    if digit > digits_vec[vec_i].0 {
                        digits_vec[vec_i] = (digit, i);
                        if digit == 9 {
                            break;
                        }
                    }
                }
            }
            let pack_sum: String = digits_vec.iter().map(|(d, _)| d.to_string()).collect();
            sum += pack_sum.parse::<u64>().unwrap();
            digits_vec.fill((0, 0));
        }
        sum
    }
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

        let day_three = DayThreeSolution { data: test_vec };
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

        let day_four = DayThreeSolution { data: test_vec };
        let sol = day_four.part_two();

        assert_eq!(3121910778619, sol);
    }
}
