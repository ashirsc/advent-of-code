use std::io::prelude::*;

fn main() {
    let file_path = "inputs/10.txt";

    let mut file = std::fs::File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let mut register:i32 = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();
    let mut processing = false;
    let mut inc = 0;
    let mut row = String::new();
    let mut output:Vec<String> = vec![];

    let mut lines = contents.lines();
    print_sprite(&register);


    for cycle in 0..240 {
        if cycle % 40 == 0 {
            output.push(row);
            row = String::new();
        }

        //draw pixel
        let mut pixel = ' ';
        if row.len() as i32 >= register - 1 && row.len() as i32 <= register  + 1 {
            pixel = '#'
        }

        row.push(pixel);
        
        if processing {
            register += inc;
            processing = false;
            print_sprite(&register);
        } else {
            let mut cmd = match lines.next() {
                Some(l) => l.split_whitespace(),
                None => break,
            };

            match cmd.next() {
                Some("addx") => {
                    inc = cmd.next().unwrap().parse().unwrap();
                    processing = true;
                }
                Some("noop") => (),
                _ => panic!("Unknown instruction"),
            }

        }
    }
    output.push(row);

    println!("final output");
   for l in output {
    println!("{}", l);
   }
}

fn print_sprite(x: &i32) {
    let mut row = String::new();
    for i in 0..40 {
        if i >= x - 1 && i <= x + 1 {
            row.push('#')
        } else {
            row.push('.')
        }
    }

    println!("{}", row);
}
