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
        let vec: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("failed to parse chars to u8s"))
            .collect();
        grid.push(vec);
    }

    let max_scenic_score = find_max_scenic_score(&grid);
    println!("Max scenic score: {}", max_scenic_score);
}

fn find_max_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
    let mut max_scenic_score = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[0].len() - 1 {
            let scenic_score = calculate_scenic_score(grid, row, col);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn calculate_scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let height = grid[row][col];

    let mut up = (0..row)
        .rev()
        .take_while(|&r| grid[r][col] < height)
        .count();
    if up < (0..row).count() {
        up+=1;
    }
    let mut down = (row + 1..grid.len())
        .take_while(|&r| grid[r][col] < height)
        .count() ;
    if down < (row + 1..grid.len()).count() {
        down += 1;
    }
    let mut left = (0..col)
        .rev()
        .take_while(|&c| grid[row][c] < height)
        .count() ;
    if left < (0..col).count() {
        left += 1;
    }
    let mut right = (col + 1..grid[0].len())
        .take_while(|&c| grid[row][c] < height)
        .count() ;
    if right < (col + 1..grid[0].len()).count() {
        right += 1;
    }

    (up * down * left * right) as u32
}
