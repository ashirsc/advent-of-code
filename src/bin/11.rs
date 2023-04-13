use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _monkey_type = lines.next().ok_or("Missing monkey type")?;

        let starting_items_line = lines.next().unwrap();

        let starting_items = starting_items_line
            .trim()
            .trim_start_matches("Starting items:")
            .trim()
            .split(',')
            // .collect();
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let operation = lines
            .next()
            .ok_or("Missing operation")?
            .split_whitespace()
            .collect::<Vec<_>>();

        let operation = match operation.as_slice() {
            ["Operation:", "new", "=", "old", "*", n] => match n {
                &"old" => Operation::Square,
                _ => {
                    let n = n
                        .parse()
                        .map_err(|_| "Invalid operation multiplier")?;
                    Operation::Multiply(n)
                }
            },
            ["Operation:", "new", "=", "old", "+", n] => {
                let n = n.parse().map_err(|_| "Invalid operation addition")?;
                Operation::Add(n)
            }
            _ => return Err("Invalid operation"),
        };

        let test = lines
            .next()
            .ok_or("Missing test")?
            .split_whitespace()
            .last()
            .expect("Test should end with a number")
            .parse::<u64>()
            .map_err(|_| "Invalid test")?;

        let if_true = lines
            .next()
            .ok_or("Missing if_true")?
            .split_whitespace()
            .last()
            .ok_or("Invalid if_true format")?
            .parse::<usize>()
            .map_err(|_| "Invalid if_true index")?;

        let if_false = lines
            .next()
            .ok_or("Missing if_false")?
            .split_whitespace()
            .last()
            .ok_or("Invalid if_false format")?
            .parse::<usize>()
            .map_err(|_| "Invalid if_false index")?;

        Ok(Self {
            starting_items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0
        })
    }
}

fn main() {
    let file_path = "inputs/11.txt";

    let mut file = std::fs::File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let monkeys = contents.split("\n\n");

    let mut monkeys = monkeys
        .into_iter()
        .map(|s| s.parse::<Monkey>())
        .collect::<Result<Vec<Monkey>, _>>()
        .unwrap();

   let test_product:u64 = monkeys.clone().iter().map(|m| m.test).product();
   println!("product {}", test_product);

    for _round in 0..10_000 {
        for m in 0..monkeys.len() {
            let mut monkey = monkeys[m].clone();
            for i in 0..monkey.starting_items.len() {
                let item = monkey.starting_items[i];
                let worry_lvl = match monkey.operation {
                    Operation::Add(v) => item + v,
                    Operation::Multiply(v) => item * v,
                    Operation::Square => item * item,
                };

                let worry_lvl = worry_lvl % test_product;
                // println!("worry level::{}", worry_lvl);

                
                if worry_lvl % monkey.test == 0 {
                    monkeys[monkey.if_true].starting_items.push(worry_lvl);
                } else {
                    monkeys[monkey.if_false].starting_items.push(worry_lvl);
                }
                monkey.inspections += 1;
            }
            monkey.starting_items = Vec::new();
            monkeys[m] = monkey;
        }
    }

    let mut scores = monkeys.iter().map(|m| m.inspections).collect::<Vec<u64>>();
    scores.sort();
    scores.reverse();

    //  for m in monkeys {
        println!("{}", scores[0]*scores[1])
    // }
}
