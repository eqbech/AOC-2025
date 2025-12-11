use std::collections::HashMap;
use nalgebra::base::Matrix;
use crate::Solution;

// Part One consts.
const YOU: &str = "you";
const OUT: &str = "out";

// Part Two consts.
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

#[derive(Clone)]
struct TerminalPathCache {
    lose_map: HashMap<String, Vec<bool>>,
}
impl TerminalPathCache {
    fn new(map: &HashMap<String, Vec<String>>) -> Self {
        let mut lose_map = HashMap::new();
        for (k, v) in map {
            lose_map.insert(k.clone(), vec![false; v.len()]);
        }
        TerminalPathCache {
            lose_map
        }
    }
    fn update(&mut self, visited: &[String], map: &HashMap<String, Vec<String>>) {
        for v in visited.iter().rev() {
            if let (Some(bool_nodes), Some(named_nodes)) = (self.lose_map.get_mut(v), map.get(v)) {
                if bool_nodes.iter().all(|b| *b) {
                    continue;
                }
                for (i, node) in named_nodes.iter().enumerate() {
                    if visited.contains(node) {
                        bool_nodes[i] = true;
                        return;
                    }
                }
            }
        }
    }
}

pub struct DayElevenSolution {
    data: HashMap<String, Vec<String>>,
    cache: TerminalPathCache
}
impl DayElevenSolution {
    fn build_adjacency_matrix(&self) -> (Matrix<u64, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<u64, nalgebra::Dyn, nalgebra::Dyn>>, HashMap<&str, usize>) {
        let size = self.data.len() + 1;
        let mut matrix = Matrix::<u64, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<u64, nalgebra::Dyn, nalgebra::Dyn>>::zeros(size, size);
        let mut keys: Vec<&str> = self.data.keys().map(String::as_str).collect();
        keys.push(OUT);
        let key_index: HashMap<&str, usize> = keys.iter().enumerate().map(|(i, k)| (*k, i)).collect();

        for (from_node, to_nodes) in &self.data {
            let from_index = *key_index.get(from_node.as_str()).unwrap();
            for to_node in to_nodes {
                if let Some(to_index) = key_index.get(to_node.as_str()) {
                    matrix[(from_index, *to_index)] = 1;
                }
            }
        }
        (matrix, key_index)
    }
}

fn num_paths(mut adjacency_matrix: Matrix<u64, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<u64, nalgebra::Dyn, nalgebra::Dyn>>, from: &str, to: &str, key_index: &HashMap<&str, usize>) -> u64 {
    let from_index = *key_index.get(from).unwrap();
    let to_index = *key_index.get(to).unwrap();
    adjacency_matrix[(from_index, from_index)] = 1;
    let powered_matrix = adjacency_matrix.pow(key_index.len() as u32 - 1);
    powered_matrix[(from_index, to_index)]
}

impl Solution for DayElevenSolution {
    const DAY: u8 = 11;

    fn new() -> Self {
        DayElevenSolution { data: parse_input(&Self::read_data_to_vec().unwrap()), cache: TerminalPathCache::new(&parse_input(&Self::read_data_to_vec().unwrap())) }
    }

    fn part_one(&self) -> u32 {
        let (mat, keys) = self.build_adjacency_matrix();
        return num_paths(mat, YOU, OUT, &keys) as u32;
    }

    fn part_two(&self) -> u64 {
        let (mat, keys) = self.build_adjacency_matrix();
        let p1 = num_paths(mat.clone(), SVR, DAC, &keys);
        let p2 = num_paths(mat.clone(), DAC, FFT, &keys);
        let p3 = num_paths(mat.clone(), FFT, OUT, &keys);
        let r = p1 * p2 * p3;
        let p1 = num_paths(mat.clone(), SVR, FFT, &keys);
        let p2 = num_paths(mat.clone(), FFT, DAC, &keys);
        let p3 = num_paths(mat.clone(), DAC, OUT, &keys);
        let l = p1 * p2 * p3;
        assert! (r == 0 || l == 0, "Either right or left path must be zero");
        return r + l;
    }
}

fn parse_input(input: &[String]) -> HashMap<String, Vec<String>> {
    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let parts: Vec<&str> = line.split(":").collect();
        let input = parts[0].trim().to_string();
        let outputs = parts[1].trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        data.insert(input, outputs);
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_11.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_eleven = DayElevenSolution {
            data: parse_input(&test_vec),
            cache: TerminalPathCache::new(&parse_input(&test_vec))
        };
        let sol = day_eleven.part_one();
        assert_eq!(5, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_11_2.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_eleven = DayElevenSolution {
            data: parse_input(&test_vec),
            cache: TerminalPathCache::new(&parse_input(&test_vec))
        };
        let sol = day_eleven.part_two();

        assert_eq!(2, sol);
    }
}
