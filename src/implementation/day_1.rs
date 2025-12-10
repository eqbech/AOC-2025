use crate::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

pub struct DayOneSolution {
    data: Vec<(Direction, i32)>,
}

impl Solution for DayOneSolution {
    const DAY: u8 = 1;

    fn new() -> Self {
        DayOneSolution {
            data: parse_input(&Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u32 {
        let mut count: u32 = 0;
        let mut position: i32 = 50;
        for (dir, amount) in &self.data {
            position += if *dir == Direction::R {
                *amount
            } else {
                -*amount
            };
            count += (position % 100 == 0) as u32;
        }
        count
    }

    fn part_two(&self) -> u32 {
        let mut count = 0;
        let mut position: i32 = 50 + (i32::MAX / 200) * 100;
        for (dir, amount) in &self.data {
            let old_position = position;
            position += if *dir == Direction::R {
                *amount
            } else {
                -*amount
            };
            let start = old_position.min(position);
            let end = old_position.max(position);
            count += ((end - start) / 100) as u32;
            count += (100 < start % 100 + ((end - start) % 100)) as u32;
            count += (position % 100 == 0) as u32;
        }
        count as u32
    }
}

fn parse_input(input: &[String]) -> Vec<(Direction, i32)> {
    input
        .iter()
        .map(|s| {
            let parts: Vec<char> = s.chars().collect();
            let dir = match parts[0] {
                'L' => Direction::L,
                'R' => Direction::R,
                _ => panic!("Unknown direction: {}", parts[0]),
            };
            let amount: i32 = parts[1..].iter().collect::<String>().parse().unwrap();
            (dir, amount)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_1.txt")
            .map(|data| {
                data.lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap();

        let day_four = DayOneSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_four.part_one();

        assert_eq!(3, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_1.txt")
            .map(|data| {
                data.lines()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap();

        let day_four = DayOneSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_four.part_two();

        assert_eq!(6, sol);
    }
}
