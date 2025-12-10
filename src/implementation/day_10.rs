use std::collections::HashSet;

use crate::Solution;

#[derive(Debug, Clone)]
struct Manual {
    lights: Vec<bool>,
    lights_goal: Vec<bool>,
    buttons: Vec<Vec<u32>>,
    joltage: Vec<u32>,
    joltage_goal: Vec<u32>,
}

impl Manual {
    fn new(lights_goal: Vec<bool>, buttons: Vec<Vec<u32>>, joltage_goal: Vec<u32>) -> Self {
        let lights = vec![false; lights_goal.len()];
        let joltage = vec![0; joltage_goal.len()]; // Placeholder, not used in current logic
        Manual {
            lights,
            lights_goal,
            buttons,
            joltage,
            joltage_goal,
        }
    }

    fn get_min_presses(&mut self) -> u32 {
        let mut presses = 1;

        for n in presses..=self.buttons.len() as u32 {
            if self.lights == self.lights_goal {
                break;
            }
            let button_combos = self.check_combinations(n as usize, 0, &mut Vec::new());
            if button_combos.0 {
                presses = n;
                break;
            }
        }
        presses
    }

    fn get_min_presses_joltage(&mut self) -> u32 {
        let mut min_p = u32::MAX;
        for n in 1..=self.buttons.len() as u32 {
            let button_combos = self.combinations(n as usize);
            for combo in button_combos {
                let (jolt, maxed_out, presses) = self.pre_process_joltage(&combo);
                let (matched, presses) =
                    self.press_buttons_joltage(&combo, jolt, maxed_out, presses, &min_p);
                if matched && presses < min_p {
                    min_p = presses;
                    println!("New minimum presses found: {} match: {:?}", min_p, combo);
                }
            }
            if min_p < u32::MAX {
                return min_p
            }
        }
        min_p
    }

    fn pre_process_joltage(&self, combination: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>, u32) {
        let mut joltage = vec![0; self.joltage_goal.len()];
        let mut presses = 0;
        let mut maxed_out = vec![];
        for combo in combination {
            let unique_idx: Vec<&u32> = combo.iter().filter(|c1| combination.iter().filter(|c2| c2.contains(c1)).count() == 1).collect();
            if !unique_idx.is_empty() {
                let b_idxes = combination.iter().enumerate()
                    .filter(|(_, b)| unique_idx.iter().any(|&u| b.contains(u)))
                    .map(|(idx, _)| idx as u32)
                    .collect::<Vec<u32>>();
                for bid in b_idxes {
                    if !maxed_out.contains(&bid) {
                        maxed_out.push(bid);
                    }
                }
            }
            for uidx in unique_idx {
                let idx_usize = *uidx as usize;
                let value = self.joltage_goal[idx_usize];
                for idx in combo {
                    joltage[*idx as usize] = value;
                }
                presses += value;
            }
        }
        (joltage, maxed_out, presses)
    }

    fn combinations(&mut self, r: usize) -> Vec<Vec<Vec<u32>>> {
        let mut result = Vec::new();
        let mut combo = Vec::new();
        self.generate_combinations(r, 0, &mut combo, &mut result);
        result
    }

    fn is_a_valid_combination(&self, combination: &Vec<Vec<u32>>) -> bool {
        let set: HashSet<u32> = combination
            .iter()
            .flat_map(|b| b.clone())
            .collect::<HashSet<u32>>();
        if set.len() != self.joltage_goal.len() {
            return false;
        }
        for combo in combination {
            let unique_idx: Vec<&u32> = combo.iter().filter(|c1| combination.iter().filter(|c2| c2.contains(c1)).count() == 1).collect();
            if unique_idx.len() > 1 {
                let prev = self.joltage_goal[*unique_idx[0] as usize];
                for &&idx in &unique_idx[1..] {
                    if self.joltage_goal[idx as usize] != prev {
                        return false;
                    }
                }
            }
            if unique_idx.len() == 1 {
                let value = self.joltage_goal[*unique_idx[0] as usize];
                for &idx in combo {
                    if self.joltage_goal[idx as usize] < value {
                        return false;
                    }
                }
            }
            // if combo
            //     .iter()
            //     .all(|c1| combination.iter().filter(|c2| c2.contains(c1)).count() == 1)
            // {
            //     let dig = self
            //         .joltage_goal
            //         .iter()
            //         .enumerate()
            //         .filter(|(i, _)| combo.contains(&(*i as u32)))
            //         .map(|(_, a)| a)
            //         .collect::<Vec<&u32>>();
            //     let first_element = dig[0];
            //     if dig.iter().any(|&d| d != first_element) {
            //         return false;
            //     }
            // }
        }
        true
    }
    fn generate_combinations(
        &mut self,
        r: usize,
        start: usize,
        current: &mut Vec<Vec<u32>>,
        result: &mut Vec<Vec<Vec<u32>>>,
    ) {
        if current.len() == r {
            if self.is_a_valid_combination(current) {
                result.push(current.clone());
            }
            return;
        }

        for i in start..self.buttons.len() {
            current.push(self.buttons[i].clone());
            self.generate_combinations(r, i + 1, current, result);
            current.pop();
        }
    }

    fn check_combinations(
        &mut self,
        r: usize,
        start: usize,
        current: &mut Vec<Vec<u32>>,
    ) -> (bool, Option<Vec<Vec<u32>>>) {
        if current.len() == r {
            if self.press_buttons_lights(current) {
                return (true, Some(current.clone()));
            }
            return (false, None);
        }

        for i in start..self.buttons.len() {
            current.push(self.buttons[i].clone());
            let res = self.check_combinations(r, i + 1, current);
            if res.0 {
                return res;
            }
            current.pop();
        }
        (false, None)
    }

    fn press_buttons_lights(&mut self, buttons_to_press: &Vec<Vec<u32>>) -> bool {
        for button in buttons_to_press {
            for &light_index in button {
                let idx = light_index as usize;
                if idx < self.lights.len() {
                    self.lights[idx] = !self.lights[idx];
                }
            }
        }
        let equal = self.lights == self.lights_goal;
        if equal {
            self.lights = vec![false; self.lights_goal.len()]; // Reset lights after checking
            return true;
        }
        self.lights = vec![false; self.lights_goal.len()]; // Reset lights after checking
        false
    }

    fn press_buttons_joltage(
        &mut self,
        buttons_to_press: &Vec<Vec<u32>>,
        joltage: Vec<u32>,
        mut maxed_out: Vec<u32>,
        presses: u32,
        min_p: &u32,
    ) -> (bool, u32) {
        if joltage == self.joltage_goal {
            return (true, presses);
        }
        if presses > self.joltage_goal.iter().sum::<u32>() || presses >= *min_p {
            return (false, 0);
        }
        for i in 0..joltage.len() {
            if joltage[i] > self.joltage_goal[i] {
                return (false, 0);
            }
        }
        for k in 0..joltage.len() {
            if joltage[k] >= self.joltage_goal[k] {
                let button_ids = buttons_to_press.iter().enumerate()
                    .filter(|(_, b)| b.contains(&(k as u32)))
                    .map(|(idx, _)| idx as u32)
                    .collect::<Vec<u32>>();
                for bid in button_ids {
                    if !maxed_out.contains(&bid) {
                        maxed_out.push(bid);
                    }
                }
            }
        }
        for (b_idx, button) in buttons_to_press.iter().enumerate() {
            if maxed_out.contains(&(b_idx as u32)) {
                continue;
            }
            let mut jolt_clone = joltage.clone();
            for &jolt_index in button {
                let idx = jolt_index as usize;
                if idx < joltage.len() {
                    jolt_clone[idx] += 1;
                }
            }
            let res = self.press_buttons_joltage(buttons_to_press, jolt_clone, maxed_out.clone(), presses + 1, min_p);
            if res.0 {
                return res;
            }
        }
        (false, 0)
    }
}

pub struct DayTenSolution {
    data: Vec<Manual>,
}

impl Solution for DayTenSolution {
    const DAY: u8 = 10;

    fn new() -> Self {
        DayTenSolution {
            data: parse_input(&Self::read_data_to_vec().unwrap()),
        }
    }

    fn part_one(&self) -> u32 {
        // Try brute force....
        let mut num_presses = 0;
        for manual in &mut self.data.clone() {
            num_presses += manual.get_min_presses();
        }
        num_presses
    }

    fn part_two(&self) -> u64 {
        // let num_cores = std::thread::available_parallelism().unwrap().get();
        // println!("Number of available cores: {}", num_cores);
        let mut presses: u64 = 0;
        for (i, manual) in self.data.clone().iter_mut().enumerate() {
            println!("Processing manual: {}", i);
            presses += manual.get_min_presses_joltage() as u64;
        }
        presses
    }
}

fn parse_input(input: &[String]) -> Vec<Manual> {
    let mut manuals: Vec<Manual> = Vec::new();

    for line in input {
        let mut parts = line.split_whitespace();
        // Lights
        let lights = parts.next().unwrap();
        let mut light_vec = Vec::new();
        for c in lights.chars() {
            match c {
                '.' => light_vec.push(false),
                '#' => light_vec.push(true),
                _ => continue,
            }
        }
        // Buttons
        let mut buttons_vec: Vec<Vec<u32>> = Vec::new();
        let mut joltage: Vec<u32> = Vec::new();
        while let Some(next_part) = parts.next() {
            let mut c = next_part.chars();
            if c.next().unwrap() == '(' {
                let mut inner_button_vec: Vec<u32> = Vec::new();
                while let Some(button_char) = c.next() {
                    if button_char.is_digit(10) {
                        inner_button_vec.push(button_char.to_digit(10).unwrap());
                    } else if button_char == ')' {
                        break;
                    }
                }
                buttons_vec.push(inner_button_vec);
            } else {
                let mut num_str = String::new();
                while let Some(dk_char) = c.next() {
                    if dk_char.is_digit(10) {
                        num_str.push(dk_char);
                    } else if dk_char == ',' {
                        if let Ok(num) = num_str.parse::<u32>() {
                            joltage.push(num);
                        }
                        num_str.clear();
                    } 
                    else if dk_char == '}' {
                        if let Ok(num) = num_str.parse::<u32>() {
                            joltage.push(num);
                        }
                        break;
                    }
                }
            }
        }
        manuals.push(Manual::new(light_vec, buttons_vec, joltage));
    }

    manuals
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_10.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_ten = DayTenSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_ten.part_one();
        assert_eq!(7, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_10.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_ten = DayTenSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_ten.part_two();

        assert_eq!(33, sol);
    }
}
