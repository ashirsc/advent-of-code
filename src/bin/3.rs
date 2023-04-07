use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_path = "inputs/3.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut sum = 0;

    let mut rucksacks = contents.lines();

    loop {
        let group = (
            match rucksacks.next() {
                Some(rucksack) => rucksack,
                None => break,
            },
            match rucksacks.next() {
                Some(rucksack) => rucksack,
                None => break,
            },
            match rucksacks.next() {
                Some(rucksack) => rucksack,
                None => break,
            },
        );


        let mut common_element:Option<char> = None;
        for c in group.0.chars() {
            if group.1.contains(c) && group.2.contains(c) {
                common_element = Some(c);
            }
        }

        if common_element.is_none() {
            continue;
        }

        let common_element = common_element.unwrap();


        let priority = if common_element.is_lowercase() {
                    common_element as u32 - 'a' as u32 + 1
                } else {
                    common_element as u32 - 'A' as u32 + 27
                };
                //sum all the values
                sum += priority;
        
        // println!("{:?}", group);
    }

    

    println!("Sum: {}", sum);
}
