use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Movement {
    Right,
    Up,
    Left,
    Down,
}

fn main() -> io::Result<()> {
    // Change the file name to the path of your text file.
    let file_path = "inputs/12.txt";
    let mut grid = read_grid(file_path)?;
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

    // let visited = vec![vec![false; grid[0].len()]; grid.len()];
    // let paths = path_find(&grid, start, Vec::new(), end, &visited);
    // let shortest_path = find_shortest_path(paths);
    let shortest_path = bfs_shortest_path(&grid, start, end);

    if let Some(path) = shortest_path {
        println!("Shortest path: {}::{:?}", path.len(), path);
        let output_grid = generate_output(&grid, &path, start);
        for row in output_grid {
            println!("{}", row.into_iter().collect::<String>());
        }
    } else {
        println!("No valid path found");
    }

    Ok(())
}

fn bfs_shortest_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<Movement>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut visited = vec![vec![false; width]; height];
    let mut queue: VecDeque<((usize, usize), Vec<Movement>)> = VecDeque::new();
    let mut longest = Vec::<Movement>::new();

    visited[start.1][start.0] = true;
    queue.push_back((start, Vec::new()));

    let directions = [
        (1, 0, Movement::Right),
        (-1, 0, Movement::Left),
        (0, -1, Movement::Up),
        (0, 1, Movement::Down),
    ];

    while let Some((location, moves)) = queue.pop_front() {
        let (x, y) = location;

        if location == end {
            return Some(moves);
        }

        if moves.len() > longest.len() {
            longest = moves.clone();
        }

        for (dx, dy, movement) in directions.iter() {
            let new_x = (x as isize) + dx;
            let new_y = (y as isize) + dy;

            if in_grid_move(new_x, new_y, width, height) {
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if !visited[new_y][new_x] {
                    let current_char = grid[y][x];
                    let next_char = grid[new_y][new_x];
                    if (next_char as i32 - current_char as i32) <= 1 {
                        visited[new_y][new_x] = true;
                        let mut new_moves = moves.clone();
                        new_moves.push(*movement);
                        queue.push_back(((new_x, new_y), new_moves));
                    }
                }
            }
        }
    }

    let output_grid = generate_output(&grid, &longest, start);
    for row in output_grid {
        println!("{}", row.into_iter().collect::<String>());
    }
    None
}

fn path_find(
    grid: &Vec<Vec<char>>,
    location: (usize, usize),
    moves: Vec<Movement>,
    end: (usize, usize),
    visited: &Vec<Vec<bool>>,
) -> Vec<Vec<Movement>> {
    let (x, y) = location;
    let rows = grid[0].len();
    let cols = grid.len();

    if x >= rows || y >= cols || visited[y][x] {
        return Vec::new();
    }

    if x == end.0 && y == end.1 {
        return vec![moves];
    }

    let mut result: Vec<Vec<Movement>> = Vec::new();
    let current_char = grid[y][x];

    let mut new_visited = visited.clone();
    new_visited[y][x] = true;

    let directions = [
        (0, 1, Movement::Down),
        (1, 0, Movement::Right),
        (-1, 0, Movement::Left),
        (0, -1, Movement::Up),
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
                let paths = path_find(grid, (new_x, new_y), new_moves, end, &new_visited);
                result.extend(paths);
            }
        }
    }

    result
}

fn in_grid_move(new_x: isize, new_y: isize, width: usize, height: usize) -> bool {
    new_x >= 0 && new_x < (width as isize) && new_y >= 0 && new_y < (height as isize)
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

fn generate_output(
    grid: &Vec<Vec<char>>,
    path: &Vec<Movement>,
    start: (usize, usize),
) -> Vec<Vec<char>> {
    let mut current_position = start;
    let mut output_grid = vec![vec!['.'; grid[0].len()]; grid.len()];

    for movement in path {
        let (dx, dy, symbol) = match movement {
            Movement::Right => (1, 0, '>'),
            Movement::Up => (0, -1, '^'),
            Movement::Left => (-1, 0, '<'),
            Movement::Down => (0, 1, 'v'),
        };

        output_grid[current_position.1][current_position.0] = symbol;
        current_position.0 = (current_position.0 as isize + dx) as usize;
        current_position.1 = (current_position.1 as isize + dy) as usize;
    }
    output_grid[current_position.1][current_position.0] = 'E';

    output_grid
}
