use crate::Solution;

pub struct DayFiveSolution {
    data: (Vec<(u64, u64)>, Vec<u64>),
}

impl Solution for DayFiveSolution {
    const DAY: u8 = 5;

    fn new() -> Self {
        DayFiveSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u32 {
        let mut sum = 0;
        let mut ranges_sorted = self.data.0.clone();
        ranges_sorted.sort_by(|a, b| a.0.cmp(&b.0));
        for id in &self.data.1 {
            let mut l = ranges_sorted.len() / 2;
            let mut r = l + 1;
            loop {
                if *id >= ranges_sorted[l].0 && *id <= ranges_sorted[l].1 
                || *id >= ranges_sorted[r].0 && *id <= ranges_sorted[r].1 {
                    sum += 1;
                    break;
                }
                if l == 0 && r == ranges_sorted.len() - 1 {
                    break;
                }
                l = if l > 0 { l - 1 } else { 0 };
                r = if r < ranges_sorted.len() - 1 { r + 1 } else { ranges_sorted.len() - 1 };
            }
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut count = 0;
        let mut ranges_sorted = self.data.0.clone();
        ranges_sorted.sort_by(|a, b| a.0.cmp(&b.0));
        let mut max_end: u64 = 0;
        for i in 0..ranges_sorted.len() {
            if i == 0 {
                count += ranges_sorted[i].1 - ranges_sorted[i].0 + 1;
                max_end = ranges_sorted[i].1;
                continue;
            }

            if ranges_sorted[i].1 <= max_end {
                continue;
            }

            if ranges_sorted[i].0 > max_end {
                max_end = ranges_sorted[i].1;
                count += ranges_sorted[i].1 - ranges_sorted[i].0 + 1;
                continue;
            }
            count += ranges_sorted[i].1 - max_end;
            max_end = ranges_sorted[i].1;
        }
        count
    }
}

fn parse_input(input: &[String]) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ids = vec![];
    let mut switch = false;
    input.iter().for_each(|line| {
        if line.trim().is_empty() {
            switch = true;
            return;
        }
        if !switch {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();
            ranges.push((start, end));
        } else {
            let id = line.trim().parse::<u64>().unwrap();
            ids.push(id);
        }
    });
    (ranges, ids)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_5.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_five = DayFiveSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_five.part_one();
        assert_eq!(5, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_5.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_five = DayFiveSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_five.part_two();

        assert_eq!(36, sol);
    }
}