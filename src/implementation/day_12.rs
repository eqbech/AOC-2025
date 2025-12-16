use crate::Solution;
use nalgebra::{Matrix3, base::Matrix};

#[derive(Debug)]
struct Present {
    index: usize,
    shape: Matrix3<u32>,
}
impl Present {
    fn new(index: usize, shape: Vec<Vec<char>>) -> Self {
        let mut matrix_data = [[0u32; 3]; 3];
        for (i, row) in shape.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                matrix_data[i][j] = if ch == '#' { 1 } else { 0 };
            }
        }
        let shape_matrix = Matrix3::from_row_slice(&matrix_data.concat());
        Present {
            index,
            shape: shape_matrix,
        }
    }
}
#[derive(Debug)]
struct Tree {
    x: u32,
    y: u32,
    shape: Matrix<u32, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<u32, nalgebra::Dyn, nalgebra::Dyn>>,
    presents_to_fit: Vec<u32>,
}
impl Tree {
    fn new(x: u32, y: u32, presents_to_fit: Vec<u32>) -> Self {
        let shape_matrix = Matrix::<u32, nalgebra::Dyn, nalgebra::Dyn, nalgebra::VecStorage<u32, nalgebra::Dyn, nalgebra::Dyn>>::zeros(x as usize, y as usize);
        Tree {
            x,
            y,
            shape: shape_matrix,
            presents_to_fit,
        }
    }
}
pub struct DayTwelveSolution {
    presents: Vec<Present>,
    trees: Vec<Tree>
}

impl Solution for DayTwelveSolution {
    const DAY: u8 = 12;

    fn new() -> Self {
        let (presents, trees) = parse_input(&Self::read_data_to_vec().unwrap());
        DayTwelveSolution { presents, trees }
    }

    fn part_one(&self) -> u32 {
        let mut fit_counter = 0;
        for tree in &self.trees {
            let total_presents = tree.presents_to_fit.iter().sum::<u32>();
            if total_presents * 3 / tree.x <= tree.y / 3 {
                fit_counter += 1;
            }
        }
        fit_counter
    }

    fn part_two(&self) -> &str {
        "No part two solution for day twelve this year!"
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<Present>, Vec<Tree>) {
    let mut presents: Vec<Present> = Vec::new();
    let mut trees: Vec<Tree> = Vec::new();
    let mut i = 0;
    while i < input.len() {
        if input[i].contains("x") {
           let parts = input[i].split(":").collect::<Vec<&str>>();
           let left_side = parts[0].trim().split("x").collect::<Vec<&str>>();
           let x = left_side[0].trim().parse::<u32>().unwrap();
           let y = left_side[1].trim().parse::<u32>().unwrap();
           let present_values = parts[1].trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
           trees.push(Tree::new(x, y, present_values));
           i += 1;
           continue;
        } else {
            if input[i].trim().contains(":") {
                let mut shape_vec: Vec<Vec<char>> = vec![];
                for k in 1..=3 {
                    let shape = input[i + k].chars().collect::<Vec<char>>();
                    shape_vec.push(shape);
                }
                let index = input[i].chars().next().unwrap().to_digit(10).unwrap() as usize;
                presents.push(Present::new(index, shape_vec));
                i += 5;
                continue;
            }
        }
        i += 1;
    }
    (presents, trees)
}