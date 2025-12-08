use std::{collections::HashSet, hash::Hash};

use crate::Solution;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for JunctionBox {
    fn from(s: &str) -> Self {
        let coords: Vec<u64> = s
            .split(',')
            .map(|part| part.trim().parse::<u64>().unwrap())
            .collect();
        JunctionBox {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

impl JunctionBox {
    fn distance(&self, other: &JunctionBox) -> u64 {
        ((self.x as i32 - other.x as i32).abs() as u64).pow(2)
            + ((self.y as i32 - other.y as i32).abs() as u64).pow(2)
            + ((self.z as i32 - other.z as i32).abs() as u64).pow(2)
    }
}

pub struct DayEightSolution {
    data: Vec<JunctionBox>,
}

impl Solution for DayEightSolution {
    const DAY: u8 = 8;

    fn new() -> Self {
        DayEightSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u64 {
        //Scuffed but now works with both test and real input
        let num_connections = if self.data.len() == 1000 {1000} else {10};

        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let connections = find_shortest_connections(&self.data);

        let mut connections_added = 1;
        circuits.push(HashSet::from([connections[0].0, connections[0].1]));
        'outer: for set in connections.iter().skip(1) {
            if connections_added >= num_connections {
                break;
            }
            let mut i = 0;
            while i < circuits.len() {
                if circuits[i].contains(&set.0) && circuits[i].contains(&set.1) {
                    //Both already in circuit
                    connections_added += 1;
                    continue 'outer;
                }
                if circuits[i].contains(&set.0) && !circuits[i].contains(&set.1) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.1) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            connections_added += 1;
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.1) && !circuits[i].contains(&set.0) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.0) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            connections_added += 1;
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.0) || circuits[i].contains(&set.1) {
                    circuits[i].insert(set.0);
                    circuits[i].insert(set.1);
                    connections_added += 1;
                    continue 'outer;
                }
                i += 1;
            }
            circuits.push(HashSet::from([set.0, set.1]));
            connections_added += 1;
        }
        // Remember to take 3 largest circuits
        circuits.sort_by_key(|c| c.len());
        circuits.iter().rev().take(3).map(|c| c.len() as u64).product::<u64>()
    }

    fn part_two(&self) -> u64 {
        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let connections = find_shortest_connections(&self.data);

        let (mut x1, mut x2) = (connections[0].0.x, connections[0].1.x);
        circuits.push(HashSet::from([connections[0].0, connections[0].1]));
        'outer: for set in connections.iter().skip(1) {
            let mut i = 0;
            while i < circuits.len() {
                if circuits[i].contains(&set.0) && circuits[i].contains(&set.1) {
                    continue 'outer;
                }
                if circuits[i].contains(&set.0) && !circuits[i].contains(&set.1) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.1) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                                x1 = set.0.x;
                                x2 = set.1.x;
                                break 'outer;
                            }
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.1) && !circuits[i].contains(&set.0) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.0) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                                x1 = set.0.x;
                                x2 = set.1.x;
                                break 'outer;
                            }
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.0) || circuits[i].contains(&set.1) {
                    circuits[i].insert(set.0);
                    circuits[i].insert(set.1);
                    if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                        x1 = set.0.x;
                        x2 = set.1.x;
                        break 'outer;
                    }
                    continue 'outer;
                }
                i += 1;
            }
            circuits.push(HashSet::from([set.0, set.1]));
        }
        x1 * x2

    }
}

fn parse_input(input: &[String]) -> Vec<JunctionBox> {
    input
        .iter()
        .map(|line| JunctionBox::from(line.as_str()))
        .collect::<Vec<JunctionBox>>()
}

fn find_shortest_connections(boxes: &Vec<JunctionBox>) -> Vec<(JunctionBox, JunctionBox, u64)> {
    let mut connections: HashSet<(JunctionBox, JunctionBox, u64)> = HashSet::with_capacity(boxes.len());
    for a_box in boxes {
        for b_box in boxes {
            if a_box != b_box {
                let distance = a_box.distance(b_box);
                if connections.contains(&(*b_box, *a_box, distance)) || connections.contains(&(*b_box, *a_box, distance)) {
                    continue;
                }
                connections.insert((*a_box, *b_box, distance));
            }
        }
    }
    let mut sorted = connections.into_iter().map(|x: (JunctionBox, JunctionBox, u64)| x).collect::<Vec<(JunctionBox, JunctionBox, u64)>>();
    sorted.sort_by_key(|(_, _, dist)| *dist);
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_8.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_eight = DayEightSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_eight.part_one();
        assert_eq!(40, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_8.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_eight = DayEightSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_eight.part_two();

        assert_eq!(25272, sol);
    }
}

