use std::fs::File;
use std::io::prelude::*;


fn main() {
    let file_path = "inputs/8.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut grid: Vec<Vec<u32>> = Vec::new();

    for line in contents.lines() {
        let vec: Vec<u32> = line.chars().map(|c| c.to_digit(10).expect("failed to parse chars to u8s")).collect();
        grid.push(vec);
    }

    
    let visible_trees = count_visible_trees(&grid);
    println!("Visible trees: {}", visible_trees);
}
fn count_visible_trees(grid: &Vec<Vec<u32>>) -> usize {
    let mut visible_trees = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_visible(grid, row, col) {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn is_visible(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let height = grid[row][col];

    let up = (0..row).rev().all(|r| grid[r][col] < height);
    let down = (row + 1..grid.len()).all(|r| grid[r][col] < height);
    let left = (0..col).rev().all(|c| grid[row][c] < height);
    let right = (col + 1..grid[0].len()).all(|c| grid[row][c] < height);

    up || down || left || right
}
