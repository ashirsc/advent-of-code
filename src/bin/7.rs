use std::fs::File;
use std::io::prelude::*;
struct Directory {
    size: i32,
    name: String,
}

fn main() {
    let file_path = "inputs/7.txt";

    let mut file = File::open(file_path).expect("Failed to open file");

    let total_space = 70000000;
    let needed_space = 30000000;

    let mut dir_stack: Vec<Directory> = Vec::new();
    let mut all_dir: Vec<Directory> = Vec::new();

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    for line in contents.lines() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("$") => {
                match words.next() {
                    Some("ls") => (),
                    Some("cd") => match words.next() {
                        Some("..") => {
                            let child = dir_stack
                                .pop()
                                .expect("bad instructions, couldn't pop child");
                            let mut parent = dir_stack
                                .pop()
                                .expect("bad instructions, couldn't pop parent");
                            parent.size += child.size;

                            all_dir.push(child);

                            dir_stack.push(parent);
                        }
                        Some("/") => {
                            dir_stack.clear();
                            let dir = Directory {
                                size: 0,
                                name: String::from("/"),
                            };
                            dir_stack.push(dir);
                        }
                        Some(dir_name) => {
                            let dir = Directory {
                                size: 0,
                                name: String::from(dir_name),
                            };
                            dir_stack.push(dir);
                        }
                        None => panic!("Invalid command"),
                    },
                    _ => println!("Invalid command"),
                };
            }
            Some("dir") => println!("dir {}", words.next().unwrap()),
            Some(c) if matches!(c.parse::<i32>(), Ok(_)) => {
                if let Ok(num) = c.parse::<i32>() {
                    println!("file: {} has size: {}", words.next().unwrap(), num);
                    let mut dir = dir_stack.pop().expect("bad instructions");
                    dir.size += num;
                    dir_stack.push(dir);
                }
            }
            _ => println!("String does not match any of the expected patterns"),
        }
    }

    for _ in 0..dir_stack.len()-1 {
        let child = dir_stack.pop().unwrap();
        let mut parent = dir_stack.pop().unwrap();
        parent.size += child.size;

        all_dir.push(child);

        dir_stack.push(parent);
    }

    let used_space = dir_stack.last().unwrap().size;
    let availible_space = total_space - used_space;
    let space_to_delete = needed_space - availible_space;

    let mut big:Vec<i32> = all_dir.iter().filter(|dir| dir.size > space_to_delete).map(|d| d.size).collect();

    println!("{:?}",big);

    println!("{}", big.first().unwrap())
}
