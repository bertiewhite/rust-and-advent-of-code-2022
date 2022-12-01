extern crate core;

use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("data/day1.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");

    let lines = data.lines();

    let mut max = 0;
    let mut current = 0;

    for line in lines {
        println!("{}", line);
        println!("{}", line.len());
        if line.len() == 0 {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap()
    }

    println!("{}", max);

    Ok(())
}
