use std::io::prelude::*;

fn main() {
    let file_path = "inputs/10.txt";

    let mut file = std::fs::File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut register = 1;
    let mut signal_strengths:Vec<i32> = Vec::new();
    let mut cycle_count = 1;
    let mut processing = false;
    let mut inc = 0;

    let mut lines = contents.lines();

    loop {
        if ((cycle_count - 20) % 40) == 0 {
            println!("cycle: {}, strength: {}",cycle_count, register);
            signal_strengths.push(cycle_count * register);
        }

        if processing {
            register += inc;
            cycle_count+=1;
            processing = false;
            continue;
        }

        let mut cmd = match lines.next() {
            Some(l) => l.split_whitespace(),
            None => break
        };

        match cmd.next() {
            Some("addx") => {
                inc = cmd.next().unwrap().parse().unwrap();
                processing = true;
            },
            Some("noop") => (),
            _ => panic!("Unknown instruction")
        }


        cycle_count += 1;
    }

    
    println!("Sum signal strength: {} ", signal_strengths.into_iter().sum::<i32>())
}


