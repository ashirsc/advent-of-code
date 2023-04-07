use std::fs::File;
use std::io::prelude::*;


fn main() {
    let file_path = "inputs/5.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let split_strings: Vec<&str> = contents.split("\n\n").collect();
    let initial_stacks = split_strings[0];
    let move_instructions = split_strings[1];

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for line in initial_stacks.lines() {
        for (i, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() {
                let col = i / 4;
                stacks[col].push(ch);
            }
        }
    }

    // Reverse each stack to have the elements in the correct order
    for stack in &mut stacks {
        stack.reverse();
    }

    for instruction in move_instructions.lines() {
        let words: Vec<&str> = instruction.split_whitespace().collect();

        if let (Ok(i), Ok(source), Ok(target)) = (
            words[1].parse::<usize>(),
            words[3].parse::<usize>(),
            words[5].parse::<usize>(),
        ) {
            let mut grabber = vec![];
            for _ in 0..i {
                let cr8 = stacks[source-1].pop().unwrap();
                grabber.push(cr8);
            }
            grabber.reverse();
            stacks[target-1].append(&mut grabber);
        } else {
            panic!("Error: Invalid numbers in the command");
        }

    }

    let mut tops = vec![];
    for stack in stacks {
        tops.push(stack[stack.len() - 1]);
    }

    println!("{:?}", tops);
}
