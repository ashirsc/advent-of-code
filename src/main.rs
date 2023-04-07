// use std::collections::BTreeSet;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
// use std::file::

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut file = File::open("inputs/day-one.txt").await?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).await?;
//     // println!("{}", contents);

//     let mut highest = BTreeSet::new();
//     let mut total = 0;

//     for line in contents.lines() {
//         if let Ok(num) = line.parse::<i32>() {
//             //the line contains a number
//             total += num;
//         } else if line.trim().is_empty() {
//             //the line is empty, decide if we should add it to the highest

//             if highest.len() < 4 {
//                 highest.insert(total);
//             } else {
//                 // let mut top_three = highest.iter().rev().take(3);
//                 let mut iter = highest.iter().rev();
//                 iter.nth(2);
//                 if total > *iter.next().unwrap() {
//                     highest.insert(total);
//                 }
//             }

//             total = 0;
//         }
//     }

//     //add together the highest three numbers
//     let mut top_three = highest.iter().rev().take(3);
//     let mut sum = 0;
//     while let Some(num) = top_three.next() {
//         sum += num;
//     }
//     println!("Sum of highest three: {}", sum);

//     Ok(())
// }

use std::fs::File;
use std::io::prelude::*;


enum Move {
    Rock,
    Paper,
    Scissors,
}

enum MoveResult {
    Win,
    Lose,
    Draw,
}

fn get_my_move_result(opp_move: &Move, my_move: &Move) -> MoveResult {
    match (opp_move, my_move) {
        (Move::Rock, Move::Rock) => MoveResult::Draw,
        (Move::Rock, Move::Paper) => MoveResult::Win,
        (Move::Rock, Move::Scissors) => MoveResult::Lose,
        (Move::Paper, Move::Rock) => MoveResult::Lose,
        (Move::Paper, Move::Paper) => MoveResult::Draw,
        (Move::Paper, Move::Scissors) => MoveResult::Win,
        (Move::Scissors, Move::Rock) => MoveResult::Win,
        (Move::Scissors, Move::Paper) => MoveResult::Lose,
        (Move::Scissors, Move::Scissors) => MoveResult::Draw,
    }
}

// Your total score is the sum of your scores for each round.

// The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

fn main() {
    let file_path = "inputs/day-two.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut score = 0;

    for line in contents.lines() {
        //parse the first two characters of the line
        let mut iter = line.split_whitespace();
        let opp_move = match iter.next().unwrap() {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Invalid move"),
        };
        let desired_result = match iter.next().unwrap() {
            "X" => MoveResult::Lose,
            "Y" => MoveResult::Draw,
            "Z" => MoveResult::Win,
            _ => panic!("Invalid result"),
        };

        let my_move = get_my_move(&opp_move, desired_result);

        let result_score = match get_my_move_result(&opp_move, &my_move) {
            MoveResult::Win => 6,
            MoveResult::Lose => 0,
            MoveResult::Draw => 3,
        };

        let move_score = match my_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        score += result_score + move_score;
    }

    println!("Score: {}", score);
}

fn get_my_move(opp_move: &Move, desired_result: MoveResult) -> Move {
    
    match (opp_move, desired_result) {
        (Move::Rock, MoveResult::Lose) => Move::Scissors,
        (Move::Rock, MoveResult::Draw) => Move::Rock,
        (Move::Rock, MoveResult::Win) => Move::Paper,
        (Move::Paper, MoveResult::Lose) => Move::Rock,
        (Move::Paper, MoveResult::Draw) => Move::Paper,
        (Move::Paper, MoveResult::Win) => Move::Scissors,
        (Move::Scissors, MoveResult::Lose) => Move::Paper,
        (Move::Scissors, MoveResult::Draw) => Move::Scissors,
        (Move::Scissors, MoveResult::Win) => Move::Rock,
    }
}
