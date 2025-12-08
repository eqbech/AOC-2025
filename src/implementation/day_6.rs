use crate::Solution;

pub struct DaySixSolution {
    data: Vec<String>,
}

impl Solution for DaySixSolution {
    const DAY: u8 = 6;

    fn new() -> Self {
        DaySixSolution { data: Self::read_data_to_vec().unwrap() }
    }

    fn part_one(&self) -> u64 {
        let data = parse_input_1(&self.data);
        let mut sum = 0u64;
        for col in &data {
            let operation = col[0];
            let mut intermediate_sum = operation;
            for &val in col.iter().skip(1) {
                if operation == 1 {
                    intermediate_sum *= val;
                } else {
                    intermediate_sum += val;
                }
            }
            sum += intermediate_sum;
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut sum = 0u64;
        let data = parse_input_2(&self.data);
        
        for col in &data {
            let operation = col.last().unwrap().chars().filter(|c| !c.is_whitespace()).next().unwrap();
            let mut res = if operation == '+' { 0 } else { 1 };
            let mut val_arr = (0..col.first().unwrap().len()).map(|_| "".to_string()).collect::<Vec<String>>();
            
            for &val in col.iter().take(col.len() - 1) {
                for (i, c) in val.chars().enumerate() {
                    if c.is_whitespace() {
                        continue;
                    }
                    val_arr[i].push(c);
                }
            }
            for val_str in val_arr {
                let val = val_str.parse::<u64>().unwrap();
                if operation == '+' {
                    res += val;
                } else {
                    res *= val;
                }
            }
            sum += res;
        }
        sum
    }
}

fn parse_input_1(input: &[String]) -> Vec<Vec<u64>> {
    let n_cols = input[0].trim().split_whitespace().collect::<Vec<&str>>().len();
    let mut res: Vec<Vec<u64>> = (0..n_cols).map(|_| Vec::new()).collect();
    
    input.iter().enumerate().for_each(|(index, line)| {
        line.trim().split_whitespace().enumerate()
            .for_each(|(col_index, num)| {
                if index == input.len() - 1 {
                    let val = match num {
                        "+" => 0,
                        "*" => 1,
                        _ => panic!("Unexpected symbol in the last line"),
                    };
                    res[col_index].push(val);
                } else {
                    let val = num.parse::<u64>().unwrap();
                    res[col_index].push(val);
                }
            })
    });
    res.iter_mut().for_each(|col| col.reverse());
    res
}

fn parse_input_2(input: &[String]) -> Vec<Vec<&str>> {
    // If all the lines have a white space in the same position, split by that position
    let split_positions: Vec<usize> = (0..input[0].len())
        .filter(|&i| input.iter().all(|line| line.as_bytes()[i] == b' '))
        .collect();
    let mut res: Vec<Vec<&str>> = (0..split_positions.len() + 1).map(|_| Vec::new()).collect();

    // println!("Input line1: {:?}", &input[0][0..split_positions[0]]);
    // println!("Split positions: {:?}", split_positions);
    input.iter().for_each(|line| {
        let mut prev_split = 0;
        for col in 0..split_positions.len() {
            let part = &line[prev_split..split_positions[col]];
            res[col].push(part);
            prev_split = split_positions[col] + 1;
        }
        let part = &line[prev_split..];
        res[split_positions.len()].push(part);
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_6.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_six = DaySixSolution {
            data: test_vec,
        };
        let sol = day_six.part_one();
        assert_eq!(4277556, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_6.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_six = DaySixSolution {
            data: test_vec,
        };
        let sol = day_six.part_two();

        assert_eq!(3263827, sol);
    }
}