use crate::Solution;

const PAPER_ROLL: u8 = b'@';

pub struct DayFourSolution {
    data: Vec<Vec<u8>>,
}

impl Solution for DayFourSolution {
    const DAY: u8 = 4;

    fn new() -> Self {
        DayFourSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u32 {
        let mut _roll_count: u8 = 0;
        let mut total_rolls: u32 = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] != PAPER_ROLL {
                    continue;
                }
                _roll_count = 0;
                // Top Left
                if y > 0 && x > 0 && self.data[y - 1][x - 1] == PAPER_ROLL {
                    _roll_count += 1;
                }
                // Top
                if y > 0 && self.data[y - 1][x] == PAPER_ROLL {
                    _roll_count += 1;
                }
                // Top Right
                if y > 0 && x < self.data[y].len() - 1 && self.data[y - 1][x + 1] == PAPER_ROLL {
                    _roll_count += 1;
                }
                // Left
                if x > 0 && self.data[y][x - 1] == PAPER_ROLL {
                    if _roll_count == 3 {
                        continue;
                    }
                    _roll_count += 1;
                }
                // Right
                if x < self.data[y].len() - 1 && self.data[y][x + 1] == PAPER_ROLL {
                    if _roll_count == 3 {
                        continue;
                    }
                    _roll_count += 1;
                }
                // Bottom Left
                if y < self.data.len() - 1 && x > 0 && self.data[y + 1][x - 1] == PAPER_ROLL {
                    if _roll_count == 3 {
                        continue;
                    }
                    _roll_count += 1;
                }
                // Bottom
                if y < self.data.len() - 1 && self.data[y + 1][x] == PAPER_ROLL {
                    if _roll_count == 3 {
                        continue;
                    }
                    _roll_count += 1;
                }
                // Bottom Right
                if y < self.data.len() - 1 && x < self.data[y].len() - 1 && self.data[y + 1][x + 1] == PAPER_ROLL {
                    if _roll_count == 3 {
                        continue;
                    }
                }
                total_rolls += 1;

            }
        }

        total_rolls as u32
    }

    fn part_two(&self) -> u32 {
        let mut all_iter_rolls = 0;
        let mut grid = self.data.clone();
        loop {
            let mut _roll_count: u8 = 0;
            let mut total_rolls: u32 = 0;
            for y in 0..grid.len() {
                for x in 0..grid[y].len() {
                    if grid[y][x] != PAPER_ROLL {
                        continue;
                    }
                    _roll_count = 0;
                    // Top Left
                    if y > 0 && x > 0 && grid[y - 1][x - 1] == PAPER_ROLL {
                        _roll_count += 1;
                    }
                    // Top
                    if y > 0 && grid[y - 1][x] == PAPER_ROLL {
                        _roll_count += 1;
                    }
                    // Top Right
                    if y > 0 && x < grid[y].len() - 1 && grid[y - 1][x + 1] == PAPER_ROLL {
                        _roll_count += 1;
                    }
                    // Left
                    if x > 0 && grid[y][x - 1] == PAPER_ROLL {
                        if _roll_count == 3 {
                            continue;
                        }
                        _roll_count += 1;
                    }
                    // Right
                    if x < grid[y].len() - 1 && grid[y][x + 1] == PAPER_ROLL {
                        if _roll_count == 3 {
                            continue;
                        }
                        _roll_count += 1;
                    }
                    // Bottom Left
                    if y < grid.len() - 1 && x > 0 && grid[y + 1][x - 1] == PAPER_ROLL {
                        if _roll_count == 3 {
                            continue;
                        }
                        _roll_count += 1;
                    }
                    // Bottom
                    if y < grid.len() - 1 && grid[y + 1][x] == PAPER_ROLL {
                        if _roll_count == 3 {
                            continue;
                        }
                        _roll_count += 1;
                    }
                    // Bottom Right
                    if y < grid.len() - 1 && x < grid[y].len() - 1 && grid[y + 1][x + 1] == PAPER_ROLL {
                        if _roll_count == 3 {
                            continue;
                        }
                    }
                    total_rolls += 1;
                    grid[y][x] = b'.';
                }
            }
            if total_rolls == 0 {
                break;
            }
            all_iter_rolls += total_rolls;
        }

        all_iter_rolls
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
        let test_string = fs::read_to_string("data/test/test_4.txt").unwrap();

        let day_four = DayFourSolution {
            data: parse_input(&test_string.lines().map(|s| s.to_string()).collect()),
        };
        let sol = day_four.part_one();
        assert_eq!(13, sol);
    }

    #[test]
    fn test_part_two() {
        let test_string = fs::read_to_string("data/test/test_4.txt").unwrap();

        let day_four = DayFourSolution {
            data: parse_input(&test_string.lines().map(|s| s.to_string()).collect()),
        };
        let sol = day_four.part_two();

        assert_eq!(43, sol);
    }
}