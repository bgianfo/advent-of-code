use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct BoxResult
{
    has_twice: bool,
    has_thrice: bool,
}

fn classify_box(box_str : String) -> BoxResult {

    let mut char_histogram = BTreeMap::new();

    for ch in box_str.chars() {

        // Fetch char's entry, or insert a zero count entry.
        //
        let entry = char_histogram.entry(ch).or_insert(0);

        // Increment the entry count.
        //
        *entry += 1
    }

    let mut result = BoxResult {
        has_twice: false,
        has_thrice: false
    };

    for count in char_histogram.values() {

        if *count == 2 {
            result.has_twice = true;
        } else if *count == 3 {
            result.has_thrice = true;
        }
    }

    return result;
}

#[test]
fn test_examples_dupes() {

    // Lambda to reduce code dupe for test cases.
    //
    fn test_classify_box(test_input: &str, expected_result : BoxResult) {

        let test_result = classify_box(test_input.to_string());

        assert_eq!(test_result.has_twice, expected_result.has_twice);
        assert_eq!(test_result.has_thrice, expected_result.has_thrice);
    }

    // "abcdef" contains no letters that appear exactly two or three times.
    //
    test_classify_box("abcdef", BoxResult { has_twice: false, has_thrice: false });

    // "bababc" contains two a and three b, so it counts for both.
    //
    test_classify_box("bababc", BoxResult { has_twice: true, has_thrice: true });

    // "abbcde" contains two b, but no letter appears exactly three times.
    //
    test_classify_box("abbcde", BoxResult { has_twice: true, has_thrice: false });

    // "abcccd" contains three c, but no letter appears exactly two times.
    //
    test_classify_box("abcccd", BoxResult { has_twice: false, has_thrice: true });

    // "aabcdd" contains two a and two d, but it only counts once.
    //
    test_classify_box("aabcdd", BoxResult { has_twice: true, has_thrice: false });

    // "abcdee" contains two e.
    //
    test_classify_box("abcdee", BoxResult { has_twice: true, has_thrice: false });

    // "ababab" contains three a and three b, but it only counts once.
    //
    test_classify_box("ababab", BoxResult { has_twice: false, has_thrice: true });
}

fn checksum_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut twice_count = 0;
    let mut thrice_count = 0;
    for line in reader.lines() {
        let result = classify_box(line.unwrap());

        if result.has_twice {
            twice_count += 1;
        }

        if result.has_thrice {
            thrice_count += 1;
        }
    }

    println!("File checksum: {}", twice_count * thrice_count);

    Ok(())
}

fn char_diff(lhs: &str, rhs : &str) -> u8 {

    let mut diff = 0;

    let zip_iter = lhs.chars().zip(rhs.chars());

    for (chlhs, chrhs) in zip_iter {
        if chlhs != chrhs {
            diff += 1;
        }
    }

    return diff;
}

#[test]
fn test_char_diff() {

    // Lambda to reduce code dupe for test cases.
    //
    fn test_diff(input_rhs: &str, input_lhs: &str, expected_diff : u8) {

        let test_diff = char_diff(input_lhs, input_rhs);

        assert_eq!(test_diff, expected_diff);
    }

    test_diff("abcde","axcye", 2);
    test_diff("fghij","fguij", 1);
}


fn same_chars(lhs: &str, rhs : &str) -> String {

    let mut diff = String::from("");

    let zip_iter = lhs.chars().zip(rhs.chars());

    for (chlhs, chrhs) in zip_iter {
        if chlhs == chrhs {
            diff.push(chlhs);
        }
    }

    return diff;
}

#[test]
fn test_same_chars() {

    // Lambda to reduce code dupe for test cases.
    //
    fn test_same(input_rhs: &str, input_lhs: &str, expected_same : &str) {

        let test_same = same_chars(input_lhs, input_rhs);

        assert_eq!(test_same, expected_same);
    }

    test_same("abcde","axcye", "ace");
    test_same("fghij","fguij", "fgij");
}

fn search_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut boxes = HashSet::new();

    for line in reader.lines() {
        boxes.insert(line.unwrap());
    }

    // Find strings which are only different by one char.
    //
    for line_x in &boxes {
        for line_y in &boxes {

            // Don't bother comparing a string with it self.
            //
            if line_x != line_y {
                let diff = char_diff(&line_x, &line_y);

                if diff == 1 {

                    let same = same_chars(line_x, line_y);
                    println!("String diff of \"{}\" and \"{}\"  is \"{}\".", line_x, line_y, same);

                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let filename = &args[2];

    println!("Mode: {}, File: {}", mode, filename);

    if mode == "checksum" {

        match checksum_file(filename) {
            Ok(_n) => println!("Success"),
            Err(_e) => println!("Error"),
        }

    } else if mode == "search" {

        match search_file(filename) {
            Ok(_n) => println!("Success"),
            Err(_e) => println!("Error"),
        }

    } else {
        println!("Unknown mode: {}", mode);
    }
}
