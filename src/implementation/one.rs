use crate::Solution;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

pub struct DayOneSolution {
    data: Vec<(Direction, u32)>,
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
            match dir {
                Direction::L => {
                    position -= *amount as i32;
                }
                Direction::R => {
                    position += *amount as i32;
                }
            }
            if (position == 0) || (position.abs() % 100 == 0) {
                count += 1;
            }
        }
        count
    }

    fn part_two(&self) -> u32 {
        let mut count = 0;
        let mut position: i32 = 50;
        for (dir, amount) in &self.data {
            match dir {
                Direction::L => {
                    let new_pos = if position == 0 {*amount as i32} else {*amount as i32 + (100 - position)};
                    count += new_pos / 100;
                    position = 100 - (new_pos % 100);
                }
                Direction::R => {
                    let new_pos = if position == 100 {*amount as i32} else {position + *amount as i32};
                    count += new_pos / 100;
                    position = new_pos % 100;

                }
            }
        }
        count as u32
    }
}

fn parse_input(input: &[String]) -> Vec<(Direction, u32)> {
    input.iter().map(|s| {
        let parts: Vec<char> = s.chars().collect();
        let dir = match parts[0] {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("Unknown direction: {}", parts[0]),
        };
        let amount: u32 = parts[1..].iter().collect::<String>().parse().unwrap();
        (dir, amount)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = 
            fs::read_to_string("data/test/day_1.txt")
                .map(|data| {
                    data.lines()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap();

        let day_four = DayOneSolution { data: parse_input(&test_vec) };
        let sol = day_four.part_one();

        assert_eq!(3, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = 
            fs::read_to_string("data/test/day_1.txt")
                .map(|data| {
                    data.lines()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>()
                })
                .unwrap();

        let day_four = DayOneSolution { data: parse_input(&test_vec) };
        let sol = day_four.part_two();

        assert_eq!(6, sol);
    }
}