mod day01;

use std::convert::{TryFrom, TryInto};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Day(i32);

impl TryFrom<i32> for Day {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || 25 < value {
            Err("Day must be in the range 1 to 25, inclusive".to_string())
        } else {
            Ok(Day(value))
        }
    }
}

impl From<Day> for i32 {
    fn from(day: Day) -> Self {
        day.0
    }
}

impl From<&Day> for i32 {
    fn from(day: &Day) -> Self {
        day.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Err("usage: aoc2020 DAY-NUM INPUT-FILENAME".to_string())?
    } else {
        let day: Day = i32::from_str(&args[1])?.try_into()?;
        let filename = generate_filename(&day);

        match day {
            Day(1) => {
                println!("01:01 => {}", crate::day01::part01(&filename)?);
                println!("01:02 => {}", crate::day01::part02(&filename)?);
            }
            _ => println!("No solution for day {}", i32::from(&day)),
        }

        Ok(())
    }
}

fn generate_filename(day: &Day) -> PathBuf {
    PathBuf::from(format!("data/day{:02}.txt", i32::from(day)))
}