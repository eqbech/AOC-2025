use std::collections::{HashMap, HashSet};

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

impl Solution for DayElevenSolution {
    const DAY: u8 = 11;

    fn new() -> Self {
        DayElevenSolution { data: parse_input(&Self::read_data_to_vec().unwrap()), cache: TerminalPathCache::new(&parse_input(&Self::read_data_to_vec().unwrap())) }
    }

    fn part_one(&self) -> u32 {
        let visited: HashSet<String> = HashSet::new();
        let mut win_map: HashMap<String, u32> = HashMap::new();
        let mut cache = self.cache.clone();
        tree_search_node_to_node(&self.data, YOU.to_string(), OUT, visited, &mut win_map, &mut cache);
        *win_map.get(YOU).unwrap()
    }

    fn part_two(&self) -> u32 {
        let visited: HashSet<String> = HashSet::new();
        let mut win_map: HashMap<String, u32> = HashMap::new();
        let mut cache = self.cache.clone();
        tree_search_node_to_node_v2(&self.data, SVR.to_string(), OUT, visited, &mut win_map , &mut cache, 0, &mut None, &mut None);
        println!("{:?}", win_map);
        *win_map.get(SVR).unwrap()
    }
}

fn tree_search_node_to_node(
    map: &HashMap<String, Vec<String>>,
    current_value: String,
    goal: &str,
    mut visited_this_branch: HashSet<String>,
    win_map: &mut HashMap<String, u32>,
    terminal_cache: &mut TerminalPathCache,
) {
    if let Some(top_score) = &win_map.get(&SVR.to_string()) {
        if **top_score > 1000 {
            return;
        }
    }
    // Win condition
    if current_value == goal {
        // this path is always a Dub.
        for v in &visited_this_branch {
            let value = win_map.entry(v.clone()).or_insert(0);
            *value += 1;
        }
        return;
    }
    if goal != OUT && current_value == OUT {
        terminal_cache.update(visited_this_branch.iter().cloned().collect::<Vec<String>>().as_slice(), map);
        return;
    }
    // Lose Condition
    if visited_this_branch.contains(&current_value) {
        terminal_cache.update(visited_this_branch.iter().cloned().collect::<Vec<String>>().as_slice(), map);
        println!("Cycle detected at node {}, map: {:?}", current_value, map);
        return;
    }
    if terminal_cache.lose_map.get(&current_value).unwrap().iter().all(|b| *b) {
        return;
    }
    visited_this_branch.insert(current_value.clone());
    if let Some(next_values) = map.get(&current_value) {
        for value in next_values {
            tree_search_node_to_node(
                map,
                value.clone(),
                goal,
                visited_this_branch.clone(),
                win_map,
                terminal_cache,
            );
        }
    }
}

fn tree_search_node_to_node_v2(
    map: &HashMap<String, Vec<String>>,
    current_value: String,
    goal: &str,
    mut visited_this_branch: HashSet<String>,
    win_map: &mut HashMap<String, u32>,
    terminal_cache: &mut TerminalPathCache,
    depth: usize,
    fft_depth: &mut Option<usize>,
    dac_depth: &mut Option<usize>,
) {
    if fft_depth.is_none() {
        if current_value == FFT {
            // println!("Setting fft depth at {} for fft", depth);
            *fft_depth = Some(depth);
        }
    }
    if dac_depth.is_none() {
        if current_value == DAC {
            // println!("Setting dac depth at {} for dac", depth);
            *dac_depth = Some(depth);
        }
    }
    if let Some(dac) = dac_depth {
        if depth > *dac && !visited_this_branch.contains(&DAC.to_string()) {
            // println!("Returning at depth {} for dac", depth);
            return;
        }
    }
    if let Some(fft) = fft_depth {
        if depth > *fft && !visited_this_branch.contains(&FFT.to_string()) {
            // println!("Returning at depth {} for fft", depth);
            return;
        }
    }
    if let (Some(fft), Some(dac)) = (&fft_depth, &dac_depth) {
        if depth > *fft.max(dac) && !visited_this_branch.contains(&FFT.to_string()) && !visited_this_branch.contains(&DAC.to_string()) {
            return;
        }
    }
    // Win condition
    if current_value == goal && visited_this_branch.contains(&DAC.to_string()) && visited_this_branch.contains(&FFT.to_string()) {
        // this path is always a Dub.
        println!("Win detected: {:?}", visited_this_branch);
        for v in &visited_this_branch {
            let value = win_map.entry(v.clone()).or_insert(0);
            *value += 1;
        }
        return;
    }
    if goal != OUT && current_value == OUT {
        terminal_cache.update(visited_this_branch.iter().cloned().collect::<Vec<String>>().as_slice(), map);
        return;
    }
    // Lose Condition
    if visited_this_branch.contains(&current_value) {
        terminal_cache.update(visited_this_branch.iter().cloned().collect::<Vec<String>>().as_slice(), map);
        println!("Cycle detected at node {}, map: {:?}", current_value, map);
        return;
    }
    if let Some(v) = terminal_cache.lose_map.get(&current_value) {
        if v.iter().all(|b| *b) {
            return;
        }
    }
    visited_this_branch.insert(current_value.clone());
    if let Some(next_values) = map.get(&current_value) {
        for value in next_values {
            tree_search_node_to_node_v2(
                map,
                value.clone(),
                goal,
                visited_this_branch.clone(),
                win_map,
                terminal_cache,
                depth + 1,
                fft_depth,
                dac_depth,
            );
        }
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
