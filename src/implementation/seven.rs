use crate::Solution;

/// Byte marking the Starting position for our `Taychon Beam`
const S: u8 = b'S';

/// Byte marking the Splitter for the `Taychon Beam`
const SPLITTER: u8 = b'^';

pub struct DaySevenSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DaySevenSolution {
    const DAY: u8 = 7;

    fn new() -> Self {
        DaySevenSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u16 {
        let mut taychon_beams: Vec<bool> = vec![false; self.data[0].len()];
        let mut n_splits: u16 = 0;
        
        // Assumption 1: Based on test and real input the Starting position always appears first row.
        let start_pos = self.data[0].iter().position(|&c| c == S).unwrap();
        taychon_beams[start_pos] = true;

        // Assumption 2: The last line never contains a Splitter.
        // It also looks like there are only splitter every other row.
        // TBD in the future what to do about it.
        for line in self.data.iter().skip(1).take(self.data.len() - 2) {
            let splitters = line.iter()
                .enumerate()
                .filter(|&(_, &c)| c == SPLITTER)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>();
            if splitters.is_empty() {
                continue;
            }
            for &splitter_pos in &splitters {
                if taychon_beams[splitter_pos] {
                    // New beam goes to the left
                    if splitter_pos > 0 {
                        taychon_beams[splitter_pos - 1] = true;
                    }
                    // New beam goes to the right
                    if splitter_pos < taychon_beams.len() - 1 {
                        taychon_beams[splitter_pos + 1] = true;
                    }
                    n_splits += 1;
                    taychon_beams[splitter_pos] = false;
                }
            }
        }
        n_splits
    }

    fn part_two(&self) -> u16 {
        // Recursion? is slow tho...
        1
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
        let test_vec = fs::read_to_string("data/test/test_7.txt").unwrap()
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
        let test_vec = fs::read_to_string("data/test/test_7.txt").unwrap()
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
