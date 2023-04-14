use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Change the file name to the path of your text file.
    let file_path = "inputs/12.txt";
    let mut grid = read_grid(file_path)?;
    let mut visited = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    grid.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, cell)| {
            if *cell == 'S' {
                *cell = 'a';
                start = (j, i);
            } else if *cell == 'E' {
                *cell = 'z';
                end = (j, i);
            }
        });
    });

    let paths = path_find(&grid, start, Vec::new(), end, &mut visited);
    let shortest_path = find_shortest_path(paths);

    if let Some(path) = shortest_path {
        println!("Shortest path: {}::{:?}", path.len(), path);
    } else {
        println!("No valid path found");
    }

    Ok(())
}
#[derive(Clone, Copy, Debug)]
enum Movement {
    Right,
    Up,
    Left,
    Down,
}
fn path_find(
    grid: &Vec<Vec<char>>,
    location: (usize, usize),
    moves: Vec<Movement>,
    end: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<Vec<Movement>> {
    let (x, y) = location;
    let rows = grid[0].len();
    let cols = grid.len();

    if x >= rows || y >= cols || visited.contains(&location) {
        return Vec::new();
    }

    if x == end.0 && y == end.1 {
        return vec![moves];
    }

    let mut result: Vec<Vec<Movement>> = Vec::new();
    let current_char = grid[y][x];

    visited.insert(location);

    let directions = [
        (1, 0, Movement::Right),
        (-1, 0, Movement::Left),
        (0, -1, Movement::Up),
        (0, 1, Movement::Down),
    ];

    for (dx, dy, movement) in directions.iter() {
        let new_x = (x as isize) + dx;
        let new_y = (y as isize) + dy;

        if in_grid_move(new_x, new_y, rows, cols) {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            let next_char = grid[new_y][new_x];
            if (next_char as i32 - current_char as i32).abs() <= 1 {
                let mut new_moves = moves.clone();
                new_moves.push(*movement);
                // println!("{:?}", new_moves);
                let mut new_visited = visited.clone();
                let paths = path_find(grid, (new_x, new_y), new_moves, end, &mut new_visited);
                result.extend(paths);
            }
        }
    }

    // println!();
    result
}
fn in_grid_move(new_x: isize, new_y: isize, rows: usize, cols: usize) -> bool {
    new_x >= 0 && new_x < (rows as isize) && new_y >= 0 && new_y < (cols as isize)
}

fn find_shortest_path(paths: Vec<Vec<Movement>>) -> Option<Vec<Movement>> {
    paths.into_iter().min_by_key(|path| path.len())
}

fn read_grid<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok(grid)
}
