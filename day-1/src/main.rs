use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn process_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut accum: i32 = 0;

    for line in reader.lines() {
        let value = line.unwrap().parse::<i32>().unwrap();
        accum += value;
    }

    // Print the accumulation.
    //
    println!("Resulting frequency: {}", accum);

    Ok(())
}

fn process_file_duplicate_frequencies(filename: &str) -> std::io::Result<()> {

    let mut accum: i32 = 0;
    let mut frequncies = HashSet::new();

    loop {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let value = line.unwrap().parse::<i32>().unwrap();

            accum += value;

            // Detect duplicate frequencies.
            //
            if frequncies.contains(&accum) {
                println!("Duplicate frequency detected: {}", accum);
                return Ok(());
            } else {
                frequncies.insert(accum);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {

        let part = &args[1];
        let filename = &args[2];

        println!("Part: {}, File: {}", part, filename);

        if part == "one" {
            match process_file(filename) {
                Ok(_n) => println!("Success"),
                Err(_e) => println!("Error"),
            }
        } else {
            match process_file_duplicate_frequencies(filename) {
                Ok(_n) => println!("Success"),
                Err(_e) => println!("Error"),
            }
        }

    } else {
        println!("Missing input file");
    }
}
