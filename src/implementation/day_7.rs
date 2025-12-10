use crate::Solution;

/// Byte marking the Starting position for our `Taychon Beam`
const START: u8 = b'S';

/// Byte marking the Splitter for the `Taychon Beam`
const SPLITTER: u8 = b'^';

pub struct DaySevenSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DaySevenSolution {
    const DAY: u8 = 7;

    fn new() -> Self {
        DaySevenSolution {
            data: parse_input(&Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u16 {
        let mut taychon_beams: Vec<bool> = vec![false; self.data[0].len()];
        let mut n_splits: u16 = 0;

        // Assumption 1: Based on test and real input the Starting position always appears first row.
        let start_pos = self.data[0].iter().position(|&c| c == START).unwrap();
        taychon_beams[start_pos] = true;
        let mut left_bound = start_pos;
        let mut right_bound = start_pos;

        // Assumption 2: The last line never contains a Splitter.
        // It also looks like there are only splitters every other row.
        // TBD in the future what to do about it.
        for line in self
            .data
            .iter()
            .skip(2)
            .take(self.data.len() - 2)
            .step_by(2)
        {
            let mut i = left_bound;
            assert!(taychon_beams.len() == line.len());
            while i <= right_bound {
                if line[i] == SPLITTER && taychon_beams[i] {
                    taychon_beams[i - 1] = true;
                    taychon_beams[i + 1] = true;
                    n_splits += 1;
                    taychon_beams[i] = false;
                }
                if line[i] == SPLITTER {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            left_bound -= 1;
            right_bound += 1;
        }
        n_splits
    }

    fn part_two(&self) -> u64 {
        let mut taychon_beams: Vec<u64> = vec![0; self.data[0].len()];

        // Assumption 1: Based on test and real input the Starting position always appears first row.
        let start_pos = self.data[0].iter().position(|&c| c == START).unwrap();
        taychon_beams[start_pos] = 1;

        // Assumption 2: The last line never contains a Splitter.
        // It also looks like there are only splitters every other row.
        // TBD in the future what to do about it.
        for line in self
            .data
            .iter()
            .skip(2)
            .take(self.data.len() - 2)
            .step_by(2)
        {
            let mut i = 1;
            assert!(taychon_beams.len() == line.len());
            while i < line.len() - 1 {
                if line[i] == SPLITTER && 0 < taychon_beams[i] {
                    taychon_beams[i - 1] += taychon_beams[i];
                    taychon_beams[i + 1] += taychon_beams[i];
                    taychon_beams[i] = 0;
                }
                if line[i] == SPLITTER {
                    i += 2;
                } else {
                    i += 1;
                }
            }
        }
        taychon_beams.iter().sum::<u64>() as u64
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_7.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_seven = DaySevenSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_seven.part_one();
        assert_eq!(21, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_7.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_seven = DaySevenSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_seven.part_two();

        assert_eq!(40, sol);
    }
}
