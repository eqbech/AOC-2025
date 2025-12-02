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
            for i in *a..=*b {
                let str_num = i.to_string();
                // if it has odd length, invalid ID
                if str_num.len() % 2 != 0 {
                    continue;
                }
                // Check if first halv is equal to last half
                let mid = str_num.len() / 2;
                if &str_num[..mid] == &str_num[mid..] {
                    sum += i;
                }
            }
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut sum = 0;
        for (a, b) in &self.data {
            for i in *a..=*b {
                let str_num = i.to_string();
                if is_valid_id_part_two(&str_num) {
                    sum += i;
                }
            }
        }
        sum
    }

}

fn is_valid_id_part_two(id: &str) -> bool {
    if id.len() < 2 {
        return false;
    }
    let first_char = id.chars().next().unwrap();
    if id.chars().all(|c| c == first_char) {
        return true
    }
    
    let even_divisors = (2..id.len()).filter(|x| id.len() % x == 0).collect::<Vec<usize>>();
    for div in even_divisors {
        let mid = id.len() / div;
        let part = &id[..mid];
        if id.chars().collect::<Vec<char>>().chunks(mid).all(|chunk| chunk.iter().collect::<String>() == part) {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input.split(',').map(|s| {
        let mut parts = s.trim().split('-');
        let first = parts.next().unwrap().parse::<u64>().unwrap();
        let second = parts.next().unwrap().parse::<u64>().unwrap();
        (first, second)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_string = 
            fs::read_to_string("data/test/test_2.txt").unwrap();

        let day_two = DayTwoSolution { data: parse_input(&test_string) };
        let sol = day_two.part_one();
        assert_eq!(1227775554, sol);
    }

    #[test]
    fn test_part_two() {
        let test_string = 
            fs::read_to_string("data/test/test_2.txt").unwrap();

        let day_two = DayTwoSolution { data: parse_input(&test_string) };
        let sol = day_two.part_two();

        assert_eq!(4174379265, sol);
    }
}