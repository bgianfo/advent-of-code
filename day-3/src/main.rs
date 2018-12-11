use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod fabric;

// Parse a line of input to it's descriptor.
fn parse_line(line : String) -> fabric::Descriptor {

    let dim: Vec<_> = line.split("@").collect();
    let nums: Vec<_> = dim[1].split(":").collect();

    let location: Vec<_> = nums[0].split(",").collect();

    let x = location[0].trim();
    let y = location[1].trim();

    let sizes: Vec<_> = nums[1].split("x").collect();

    let h = sizes[0].trim();
    let w = sizes[1].trim();

    fabric::Descriptor {
        x: x.parse::<u32>().unwrap(),
        y: y.parse::<u32>().unwrap(),
        h: h.parse::<u32>().unwrap(),
        w: w.parse::<u32>().unwrap(),
    }
}

#[test]
fn test_parse_line() {
    let res = parse_line("#1218 @ 152,658: 11x17".to_string());
    assert_eq!(res.x, 152);
    assert_eq!(res.y, 658);
    assert_eq!(res.h, 11);
    assert_eq!(res.w, 17);
}

fn parse_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut fabric = fabric::FabricPiece::new();

    for line in reader.lines() {
        let desc = parse_line(line.unwrap());

        println!("Desc: {:?}", desc);

        fabric.populate(&desc);
    }

    println!("Double booked count: {}", fabric.double_booked_count());

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("File: {}", filename);

    match parse_file(filename) {
        Ok(_n) => println!("Success"),
        Err(_e) => println!("Error"),
    }
}
