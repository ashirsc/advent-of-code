
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("inputs/day-one.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    // println!("{}", contents);

    let mut highest = BTreeSet::new();
    let mut total = 0;

    for line in contents.lines() {
        if let Ok(num) = line.parse::<i32>() {}
    }
}
