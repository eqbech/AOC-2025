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
#[cfg(test)]
const NUM_CONNECTIONS: usize = 10;

#[cfg(not(test))]
const NUM_CONNECTIONS: usize = 1000;

impl Solution for DayEightSolution {
    const DAY: u8 = 8;

    fn new() -> Self {
        DayEightSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u64 {
        //Scuffed but now works with both test and real input

        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let connections = find_shortest_connections(&self.data);

        let mut connections_added = 0;
        'outer: for set in connections {
            if connections_added >= NUM_CONNECTIONS {
                break;
            }
            let mut i = 0;
            while i < circuits.len() {
                if circuits[i].contains(&set.box_a) && circuits[i].contains(&set.box_b) {
                    //Both already in circuit
                    connections_added += 1;
                    continue 'outer;
                }
                if circuits[i].contains(&set.box_a) && !circuits[i].contains(&set.box_b) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.box_b) {
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
                if circuits[i].contains(&set.box_b) && !circuits[i].contains(&set.box_a) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.box_a) {
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
                if circuits[i].contains(&set.box_a) || circuits[i].contains(&set.box_b) {
                    circuits[i].insert(set.box_a);
                    circuits[i].insert(set.box_b);
                    connections_added += 1;
                    continue 'outer;
                }
                i += 1;
            }
            circuits.push(HashSet::from([set.box_a, set.box_b]));
            connections_added += 1;
        }
        // Remember to take 3 largest circuits
        circuits.sort_by_key(|c| c.len());
        circuits.iter().rev().take(3).map(|c| c.len() as u64).product::<u64>()
    }

    fn part_two(&self) -> u64 {
        let mut circuits: Vec<HashSet<JunctionBox>> = Vec::new();
        let connections = find_shortest_connections(&self.data);

        let (mut x1, mut x2) = (0, 0);
        'outer: for set in connections {
            let mut i = 0;
            while i < circuits.len() {
                if circuits[i].contains(&set.box_a) && circuits[i].contains(&set.box_b) {
                    continue 'outer;
                }
                if circuits[i].contains(&set.box_a) && !circuits[i].contains(&set.box_b) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.box_b) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                                x1 = set.box_a.x;
                                x2 = set.box_b.x;
                                break 'outer;
                            }
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.box_b) && !circuits[i].contains(&set.box_a) {
                    for k in 0..circuits.len() {
                        if circuits[k].contains(&set.box_a) {
                            let (bucket_to_extend, bucket_to_remove) = (k.min(i), k.max(i));
                            let to_merge = circuits.remove(bucket_to_remove);
                            let current_set = &mut circuits[bucket_to_extend];
                            for item in to_merge {
                                current_set.insert(item);
                            }
                            if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                                x1 = set.box_a.x;
                                x2 = set.box_b.x;
                                break 'outer;
                            }
                            continue 'outer;
                        }
                    }
                }
                if circuits[i].contains(&set.box_a) || circuits[i].contains(&set.box_b) {
                    circuits[i].insert(set.box_a);
                    circuits[i].insert(set.box_b);
                    if circuits.len() == 1 && circuits[0].len() == self.data.len() {
                        x1 = set.box_a.x;
                        x2 = set.box_b.x;
                        break 'outer;
                    }
                    continue 'outer;
                }
                i += 1;
            }
            circuits.push(HashSet::from([set.box_a, set.box_b]));
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

#[derive(Eq, PartialEq, Hash)]
struct Connection {
    box_a: JunctionBox,
    box_b: JunctionBox,
    distance: u64,
}

fn find_shortest_connections(boxes: &Vec<JunctionBox>) -> impl IntoIterator<Item = Connection> {
    let mut connections: HashSet<Connection> = HashSet::with_capacity(boxes.len() * boxes.len());
    let mut index_a = 0;
    while index_a < boxes.len() {
        let mut index_b = index_a + 1;
        while index_b < boxes.len() {
            let distance = boxes[index_a].distance(&boxes[index_b]);
            connections.insert(Connection { box_a: boxes[index_a], box_b: boxes[index_b], distance });
            index_b += 1;
        }
        index_a += 1;
    }
    let mut sorted = connections.into_iter().map(|x| x).collect::<Vec<Connection>>();
    sorted.sort_by_key(|conn| conn.distance);
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

