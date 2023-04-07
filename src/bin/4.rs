use std::fs::File;
use std::io::prelude::*;

struct Range {
    high: u32,
    low: u32,
}

fn main() {
    let file_path = "inputs/4.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut overlapping_pairs = 0;

    for line in contents.lines() {
        //split the line at ,
        let ranges: Vec<Range> = line
            .split(',')
            .map(|range| {
                let parts: Vec<&str> = range.split('-').collect();
                let range = Range {
                    low: parts[0].parse().unwrap(),
                    high: parts[1].parse().unwrap(),
                };
                range
            })
            .collect();

        let partner_1:&Range = ranges.first().unwrap();
        let partner_2:&Range = ranges.last().unwrap();

        if partner_1.high >= partner_2.low && partner_2.high >= partner_1.low {
            overlapping_pairs += 1;
        }
    }
    println!("{} ", overlapping_pairs)
}
