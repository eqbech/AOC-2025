use std::collections::{HashMap, HashSet};

use crate::Solution;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn area(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + 1) as u64 * ((self.y - other.y).abs() + 1) as u64
    }
}

#[derive(Debug)]
struct TileArea<'a> {
    point_a: &'a Point,
    point_b: &'a Point,
    area: u64,
}
#[derive(Debug)]
struct Corners {
    top_left: Point,
    top_right: Point,
    bottom_left: Point,
    bottom_right: Point,
}
impl<'a> TileArea<'a> {
    /// Returns in order x1, x2, y1, y2
    fn get_edges(&self) -> Corners {
        let min_x = self.point_a.x.min(self.point_b.x);
        let max_x = self.point_a.x.max(self.point_b.x);
        let min_y = self.point_a.y.min(self.point_b.y);
        let max_y = self.point_a.y.max(self.point_b.y);

        Corners {
            top_left: Point { x: min_x, y: min_y },
            top_right: Point { x: max_x, y: min_y },
            bottom_left: Point { x: min_x, y: max_y },
            bottom_right: Point { x: max_x, y: max_y },
        }
    }
    fn get_dimensions(&self) -> (i32, i32) {
        let width = (self.point_a.x - self.point_b.x).abs() + 1;
        let height = (self.point_a.y - self.point_b.y).abs() + 1;
        (width, height)
    }
}

impl<'a> Ord for TileArea<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area.cmp(&other.area)
    }
}

impl<'a> PartialOrd for TileArea<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for TileArea<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.area == other.area &&
        self.point_a.x == other.point_a.x &&
        self.point_a.y == other.point_a.y &&
        self.point_b.x == other.point_b.x &&
        self.point_b.y == other.point_b.y
    }
}

impl<'a> Eq for TileArea<'a> {}

pub struct DayNineSolution {
    data: Vec<Point>,
}

impl Solution for DayNineSolution {
    const DAY: u8 = 9;

    fn new() -> Self {
        DayNineSolution { data: parse_input(&Self::read_data_to_vec().unwrap()) }
    }

    fn part_one(&self) -> u64 {
        let mut largest_area = 0;
        let mut y = 0;

        while y < self.data.len() {
            let mut x = y + 1;
            while x < self.data.len() {
                let area = self.data[y].area(&self.data[x]);
                if area > largest_area {
                    largest_area = area;
                }
                x += 1;
            }
            y += 1;
        }
        largest_area
    }

    fn part_two(&self) -> u64 {   
        let mut loops: Vec<Vec<Point>> = Vec::with_capacity(self.data.len());
        get_loops(&self.data[0], &self.data[0], None, &self.data, vec![], &mut loops);
        assert_eq!(loops.len(), 2);
        let mut points = loops[0].clone();

        // Remove points inbetween points on the same row / column
        let mut valid_points: Vec<Point> = Vec::with_capacity(points.len());
        valid_points.push(points[0]);
        let mut i = 1;
        while i < points.len() {
            if i + 1 >= points.len() {
                valid_points.push(points[i].clone());
                break;
            }
            let curr = &points[i];
            let prev = &valid_points[valid_points.len() - 1];
            let next = &points[i + 1];
            
            if prev.x == curr.x && curr.x == next.x {
                points.remove(i);
                continue;
            }

            if prev.y == curr.y && curr.y == next.y {
                points.remove(i);
                continue;
            }

            valid_points.push(curr.clone());
            i += 1;
        }
        let mut boundary_points: Vec<Point> = Vec::with_capacity(valid_points.len());
        for (a, b) in valid_points.iter().zip(valid_points.iter().skip(1)) {
            if a.x == b.x {
                let range = if a.y < b.y {
                    a.y..=b.y
                } else {
                    b.y..=a.y
                };
                for y in range {
                    boundary_points.push(Point { x: a.x, y });
                }
            } else if a.y == b.y {
                let range = if a.x < b.x {
                    a.x..=b.x
                } else {
                    b.x..=a.x
                };
                for x in range {
                    boundary_points.push(Point { x, y: a.y });
                }
            }
        }
        let (special_a, special_b) = (valid_points[0].clone(), valid_points[valid_points.len() -1].clone());
        if special_a.x == special_b.x {
            let range = if special_a.y < special_b.y {
                special_a.y..=special_b.y
            } else {
                special_b.y..=special_a.y
            };
            for y in range {
                boundary_points.push(Point { x: special_a.x, y });
            }
        } else if special_a.y == special_b.y {
            let range = if special_a.x < special_b.x {
                special_a.x..=special_b.x
            } else {
                special_b.x..=special_a.x
            };
            for x in range {
                boundary_points.push(Point { x, y: special_a.y });
            }
        }
        let hased: HashSet<Point> = HashSet::from_iter(boundary_points.into_iter());
        boundary_points = hased.iter().cloned().collect::<Vec<Point>>();
        let mut y_map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut x_map: HashMap<i32, (i32, i32)> = HashMap::new();
        for point in &boundary_points {
            let y_entry = y_map.entry(point.y).or_insert((i32::MAX, 0));
            if point.x < y_entry.0 {
                y_entry.0 = point.x;
            }
            if point.x > y_entry.1 {
                y_entry.1 = point.x;
            }
            let x_entry = x_map.entry(point.x).or_insert((i32::MAX, 0));
            if point.y < x_entry.0 {
                x_entry.0 = point.y;
            }
            if point.y > x_entry.1 {
                x_entry.1 = point.y;
            }
        }

        let mut largest_area = 0;
        let mut y = 0;
        while y < valid_points.len() {
            let mut x = y + 1;
            while x < valid_points.len() {
                let t = TileArea {
                    point_a: &valid_points[y],
                    point_b: &valid_points[x],
                    area: valid_points[y].area(&valid_points[x]),
                };
                if t.area <= largest_area {
                    x += 1;
                    continue;
                }
                if is_area_valid(&x_map, &y_map, &t) {
                    if t.area > largest_area {
                        largest_area = t.area;
                    }
                }
                x += 1;
            }
            y += 1;
        }
        largest_area
    }
}

fn is_area_valid(x_map: &HashMap<i32, (i32, i32)>, y_map: &HashMap<i32, (i32, i32)>, tile_area: &TileArea) -> bool {
    let edges = tile_area.get_edges();
    let (width, height) = tile_area.get_dimensions();

    // check out of bounds X
    for y in edges.top_left.y..(edges.top_left.y + height) {
        if edges.top_left.x < y_map.get(&y).unwrap().0 || edges.top_right.x > y_map.get(&y).unwrap().1{
            return false;
        }
    }
    // check out of bounds Y
    for x in edges.top_left.x..(edges.top_left.x + width) {
        if edges.top_left.y < x_map.get(&x).unwrap().0 || edges.top_right.y > x_map.get(&x).unwrap().1 {
            return false;
        }
    }

    true
}
// Todo now traverse both direction of loop only need to go one way...
fn get_loops(
    start: &Point,
    prev: &Point,
    current: Option<&Point>,
    points: &[Point],
    visited_points: Vec<Point>,
    loops: &mut Vec<Vec<Point>>,
) {
    // Check if there are points in between start and candidate that would block a straight line.
    let curr_point = match current {
        Some(p) => {
            if p == start && visited_points.len() >= 1 {
                loops.push(visited_points.clone());
                return
            }
            p
        },
        None => start,
    };
    if visited_points.len() > points.len() {
        return
    }
    // Up
    let up = points.iter().find(|p| p.x == curr_point.x && p.y > curr_point.y && *p != prev);
    if let Some(up_point) = up {
        if visited_points.contains(up_point) {
            return
        }
        let mut temp = visited_points.clone();
        temp.push(up_point.clone());
        get_loops(start, curr_point, Some(up_point), points, temp, loops);
    }
    // Down
    let down = points.iter().find(|p| p.x == curr_point.x && p.y < curr_point.y && *p != prev);
    if let Some(down_point) = down {
        if visited_points.contains(down_point) {
            return
        }
        let mut temp = visited_points.clone();
        temp.push(down_point.clone());
        get_loops(start, curr_point, Some(down_point), points, temp, loops);
    }
    // Left
    let left = points.iter().find(|p| p.x < curr_point.x && p.y == curr_point.y && *p != prev);
    if let Some(left_point) = left {
        if visited_points.contains(left_point) {
            return
        }
        let mut temp = visited_points.clone();
        temp.push(left_point.clone());
        get_loops(start, curr_point, Some(left_point), points, temp, loops);
    }
    // Right
    let right = points.iter().find(|p| p.x > curr_point.x && p.y == curr_point.y && *p != prev);
    if let Some(right_point) = right {
        if visited_points.contains(right_point) {
            return
        }
        let mut temp = visited_points.clone();
        temp.push(right_point.clone());
        get_loops(start, curr_point, Some(right_point), points, temp, loops);
    }

    return
}

fn parse_input(input: &Vec<String>) -> Vec<Point> {
    input
        .iter()
        .map(|line| {
            let coords: Vec<i32> = line
                .split(',')
                .map(|part| part.trim().parse::<i32>().unwrap())
                .collect();
            Point {
                x: coords[0],
                y: coords[1],
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let test_vec = fs::read_to_string("data/test/test_9.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_nine = DayNineSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_nine.part_one();
        assert_eq!(50, sol);
    }

    #[test]
    fn test_part_two() {
        let test_vec = fs::read_to_string("data/test/test_9.txt").unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let day_nine = DayNineSolution {
            data: parse_input(&test_vec),
        };
        let sol = day_nine.part_two();

        assert_eq!(24, sol);
    }
}