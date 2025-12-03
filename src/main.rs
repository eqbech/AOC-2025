use aoc_2025::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    AdventOfCodeSolver::new().solve(args);
}
