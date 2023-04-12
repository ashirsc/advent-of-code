use std::io::prelude::*;

fn main() {
    let file_path = "inputs/9.txt";

    let mut file = std::fs::File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut tail_visits: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
    let mut rope:Vec<(i32,i32)> = vec![(0,0); 10];
    tail_visits.insert(*rope.last().unwrap());

    for line in contents.lines() {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let mag = words
            .next()
            .expect("No magnitude provided")
            .parse()
            .unwrap();
        for _ in 0..mag {
            match direction {
                "R" => rope[0].0 += 1,
                "U" => rope[0].1 += 1,
                "L" => rope[0].0 -= 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("Invalid direction"),
            }
            for i in 1..rope.len() {
                
                let tail_move = calc_tail_move(rope[i-1], rope[i]);
                rope[i].0 += tail_move.0;
                rope[i].1 += tail_move.1;
            }
            tail_visits.insert(*rope.last().unwrap());
        }
    }
    println!("Tail at {:?}", rope.last().unwrap());
    println!("Unique tail visits: {} ", tail_visits.len())
}

fn calc_tail_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    let distance_squared = dx * dx + dy * dy;

    // If the head and tail are touching diagonally or overlapping
    if distance_squared <= 2 {
        (0, 0)
    } else if distance_squared == 4 {
        (dx / 2, dy / 2)
    } else {
        (dx.signum(), dy.signum())
    }
}
