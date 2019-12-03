//! Link: https://adventofcode.com/2019/day/1
//!
//! --- Day 1: The Tyranny of the Rocket Equation ---
//! Santa has become stranded at the edge of the Solar System while delivering presents to other planets! To accurately calculate his position in space, safely align his warp drive, and return to Earth in time to save Christmas, he needs you to bring him measurements from fifty stars.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! The Elves quickly load you into a spacecraft and prepare to launch.
//!
//! At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They haven't determined the amount of fuel required yet.
//!
//! Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
//!
//! For example:
//!
//! - For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
//! - For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
//! - For a mass of 1969, the fuel required is 654.
//! - For a mass of 100756, the fuel required is 33583.
//!
//! The Fuel Counter-Upper needs to know the total fuel requirement. 
//! To find it, individually calculate the fuel needed for the mass of each module (your puzzle input), then add together all the fuel values.
//! What is the sum of the fuel requirements for all of the modules on your spacecraft?!
//!
//!
//! Solution:
//!
//!  $ cargo run input.txt
//!    Compiling aoc-2019-day-1 v0.1.0 (C:\src\advent-of-code\2019\day-1)
//!     Finished dev [unoptimized + debuginfo] target(s) in 1.34s
//!      Running `C:\src\advent-of-code\2019\day-1\target\debug\aoc-2019-day-1.exe input.txt`
//!
//!  Parsing file: input.txt
//!  Total Fuel Required: 3443395
//!  Success
//!
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// "To find the fuel required for a module, take its mass, divide by three, round down, and subtract 2."
fn calculate_fuel_required(mass: u64) -> u64 {
    let ratio  =  (mass as f64) / 3.0f64;
    let rounded_down = ratio.floor();

    return (rounded_down - 2.0) as u64;
}

#[test]
fn test_calculate_fuel_requried() {
    assert_eq!(2, calculate_fuel_required(12));
    assert_eq!(2, calculate_fuel_required(14));
    assert_eq!(654, calculate_fuel_required(1969));
    assert_eq!(33583, calculate_fuel_required(100756));
}

fn process_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut accumulated_fuel: u64 = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse::<u64>().unwrap();
        accumulated_fuel += calculate_fuel_required(mass);
    }

    // Print the accumulation.
    //
    println!("Total Fuel Required: {}", accumulated_fuel);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {

        let filename = &args[1];

        println!("Parsing file: {}", filename);

        match process_file(filename) {
            Ok(_n) => println!("Success"),
            Err(_e) => println!("Error"),
        }
    } else {
        println!("Missing input file");
    }
}
