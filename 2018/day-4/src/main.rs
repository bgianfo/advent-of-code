use std::env;
use std::str::FromStr;
use std::string::ParseError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::option;
use regex::Regex;
use chrono::{NaiveDate, NaiveDateTime, Datelike};

// Declare a new type for our id.
//
type GuardId = u16;

// Track actions
#[derive(Debug, PartialEq)]
enum GuardAction {
    BeginShift { guard_id : GuardId },
    WakeUp,
    FallAsleep,
}

impl Default for GuardAction {
    fn default() -> GuardAction { 
        GuardAction::BeginShift { guard_id: 0 }
    }
}

#[derive(Debug)]
struct GuardEvent
{
    action : GuardAction,
    time_stamp: NaiveDateTime,
}

impl FromStr for GuardEvent {

	type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let re = Regex::new(r"(?x)
            \[
                (?P<year>\d{4})  # the year
                -
                (?P<month>\d{2}) # the month
                -
                (?P<day>\d{2})   # the day
                \s+
                (?P<hour>\d{2})   # the hour
                :
                (?P<minute>\d{2})   # the minute
            \]
            \s+
            (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
            ").unwrap();

        let captures = re.captures(&s).unwrap();

        let year = captures["year"].parse::<i32>().unwrap();
        let month = captures["month"].parse::<u32>().unwrap();
        let day = captures["day"].parse::<u32>().unwrap();

        let hour = captures["hour"].parse::<u32>().unwrap();
        let minute = captures["minute"].parse::<u32>().unwrap();

        let ts = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 0);

        let action =
            if let Some(m) = captures.name("id") {
               GuardAction::BeginShift { guard_id: m.as_str().parse::<GuardId>().unwrap() }
            } else if &captures["sleep"] == "falls asleep" {
                GuardAction::FallAsleep
            } else if &captures["sleep"] == "wakes up" {
                GuardAction::WakeUp
            } else {
                panic!("AHH!");
            };

        Ok(GuardEvent{time_stamp: ts, action: action })
    }
}

// Lambda to reduce code dupe for test cases.
//
#[cfg(test)]
fn test_parse(test_input: &str, expected_event : GuardEvent) {

    let parsed_event = GuardEvent::from_str(test_input).unwrap();

    assert_eq!(parsed_event.time_stamp, expected_event.time_stamp);
    assert_eq!(parsed_event.action, expected_event.action);
}

#[test]
fn test_parse_guard_event_begin() { 
    test_parse("[1518-11-01 00:00] Guard #10 begins shift",
               GuardEvent {
                   action: GuardAction::BeginShift { guard_id: 10 },
                   time_stamp: NaiveDate::from_ymd(1518, 11, 1).and_hms( 0, 0, 0),
               });
}

#[test]
fn test_parse_guard_event_sleep() { 
    test_parse("[1518-11-01 00:05] falls asleep",
               GuardEvent {
                   action: GuardAction::FallAsleep,
                   time_stamp: NaiveDate::from_ymd(1518, 11, 1).and_hms( 0, 5, 0),
               });
}

#[test]
fn test_parse_guard_event_wake() { 
    test_parse("[1518-11-01 00:25] wakes up",
               GuardEvent {
                   action: GuardAction::WakeUp,
                   time_stamp: NaiveDate::from_ymd(1518, 11, 1).and_hms( 0, 25, 0),
               });
}

fn minutes_guard_asleep(events: &Vec<GuardEvent>) {

    let minutes = 0;
    let last_time_stamp : Option<NaiveDateTime> = None;

    for event in events {
        

    }
}

fn parse_file(filename: &str) -> std::io::Result<()> {

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut events : Vec<GuardEvent> = vec![];
    for line in reader.lines() {
        let event = GuardEvent::from_str(&line.unwrap()).unwrap();
    
        events.push(event);
    }

    events.sort_by(|a, b| a.time_stamp.cmp(&b.time_stamp));

    type GuardEventsById = HashMap<GuardId, Vec<GuardEvent>>;

    let mut events_by_guard = GuardEventsById::new();

    let mut last_id : GuardId = 0;

    for event in events { 

        match event.action {
            GuardAction::BeginShift{guard_id} => last_id = guard_id,
            _ => (),
        }

        let bucket = events_by_guard.entry(last_id).or_insert(vec![]);

        bucket.push(event);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("File: {}", filename);

    match parse_file(filename) {
        Ok(_n) => println!("Completed with: Success."),
        Err(_e) => println!("Completed with: Error"),
    }
}
