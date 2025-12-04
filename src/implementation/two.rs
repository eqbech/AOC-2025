use crate::Solution;

pub struct DayTwoSolution {
    data: Vec<(u64, u64)>,
}

impl Solution for DayTwoSolution {
    const DAY: u8 = 2;

    fn new() -> Self {
        DayTwoSolution {
            data: parse_input(&Self::read_data_to_string().unwrap()),
        }
    }

    fn part_one(&self) -> u64 {
        let mut sum = 0;
        for (a, b) in &self.data {
            if a.to_string().len() == b.to_string().len() && a.to_string().len() % 2 != 0 {
                continue;
            }
            let mut i = *a;
            while i <= *b {
                let str_num = i.to_string();
                // if it has odd length, skip to when it is even
                if str_num.len() % 2 != 0 {
                    i = 10_u64.pow(str_num.len() as u32);
                    continue;
                }
                // Check if first half is equal to last half
                let mid = str_num.len() / 2;
                let (first, second) = (str_num[..mid].parse::<u64>().unwrap(), str_num[mid..].parse::<u64>().unwrap());
                if first == second {
                    sum += i;
                    i += 10_u64.pow(mid as u32);
                    continue;
                }
                if first > second {
                    // Move to next possible valid number
                    i += first - second;
                    continue;
                }
                // Move to next possible valid number
                i += 10_u64.pow(mid as u32) - (second - first);
            }
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut sum = 0;
        for (a, b) in &self.data {
            let mut i = *a;
            while i <= *b {
                if is_valid_id_part_two(&i.to_string().chars().collect::<Vec<char>>()) {
                    sum += i;
                }
                i += 1;
            }
        }
        sum
    }
}

fn is_valid_id_part_two(digits: &[char]) -> bool {
    if digits.len() < 2 {
        return false;
    }
    if digits.iter().all(|&c| c == digits[0]) {
        return true;
    }
    for div in 2..=digits.len() / 2 {
        if digits.len().is_multiple_of(div) && digits.chunks(div).all(|c| *c == digits[0..div]) {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|s| {
            let mut parts = s.trim().split('-');
            let first = parts.next().unwrap().parse::<u64>().unwrap();
            let second = parts.next().unwrap().parse::<u64>().unwrap();
            (first, second)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_string = fs::read_to_string("data/test/test_2.txt").unwrap();

        let day_two = DayTwoSolution {
            data: parse_input(&test_string),
        };
        let sol = day_two.part_one();
        assert_eq!(1227775554, sol);
    }

    #[test]
    fn test_part_two() {
        let test_string = fs::read_to_string("data/test/test_2.txt").unwrap();

        let day_two = DayTwoSolution {
            data: parse_input(&test_string),
        };
        let sol = day_two.part_two();

        assert_eq!(4174379265, sol);
    }
}
