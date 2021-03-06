mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;

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
        Err("usage: aoc2020 DAY-NUM".to_string())?
    } else {
        let day: Day = i32::from_str(&args[1])?.try_into()?;
        let filename = generate_filename(&day);

        match day {
            Day(1) => {
                println!("01:01 => {}", crate::day01::part01(&filename)?);
                println!("01:02 => {}", crate::day01::part02(&filename)?);
            }
            Day(2) => {
                println!("02:01 => {}", crate::day02::part01(&filename)?);
                println!("02:02 => {}", crate::day02::part02(&filename)?);
            }
            Day(3) => {
                println!("03:01 => {}", crate::day03::part01(&filename)?);
                println!("03:02 => {}", crate::day03::part02(&filename)?);
            }
            Day(4) => {
                println!("04:01 => {}", crate::day04::part01(&filename)?);
                println!("04:02 => {}", crate::day04::part02(&filename)?);
            }
            Day(5) => {
                println!("05:01 => {}", crate::day05::part01(&filename)?);
                println!("05:02 => {}", crate::day05::part02(&filename)?);
            }
            Day(6) => {
                println!("06:01 => {}", crate::day06::part01(&filename)?);
                println!("06:02 => {}", crate::day06::part02(&filename)?);
            }
            Day(7) => {
                println!("07:01 => {}", crate::day07::part01(&filename)?);
                println!("07:02 => {}", crate::day07::part02(&filename)?);
            }
            Day(8) => {
                println!("08:01 => {}", crate::day08::part01(&filename)?);
                println!("08:02 => {}", crate::day08::part02(&filename)?);
            }
            Day(9) => {
                println!("09:01 => {}", crate::day09::part01(&filename)?);
                println!("09:02 => {}", crate::day09::part02(&filename)?);
            }
            Day(10) => {
                println!("10:01 => {}", crate::day10::part01(&filename)?);
                println!("10:02 => {}", crate::day10::part02(&filename)?);
            }
            Day(11) => {
                println!("11:01 => {}", crate::day11::part01(&filename)?);
                println!("11:02 => {}", crate::day11::part02(&filename)?);
            }
            Day(12) => {
                println!("12:01 => {}", crate::day12::part01(&filename)?);
                println!("12:02 => {}", crate::day12::part02(&filename)?);
            }
            Day(13) => {
                println!("13:01 => {}", crate::day13::part01(&filename)?);
                println!("13:02 => {}", crate::day13::part02(&filename)?);
            }
            Day(14) => {
                println!("14:01 => {}", crate::day14::part01(&filename)?);
                println!("14:02 => {}", crate::day14::part02(&filename)?);
            }
            Day(15) => {
                println!("15:01 => {}", crate::day15::part01(&filename)?);
                println!("15:02 => {}", crate::day15::part02(&filename)?);
            }
            Day(16) => {
                println!("16:01 => {}", crate::day16::part01(&filename)?);
                println!("16:02 => {}", crate::day16::part02(&filename)?);
            }
            Day(17) => {
                println!("17:01 => {}", crate::day17::part01(&filename)?);
                println!("17:02 => {}", crate::day17::part02(&filename)?);
            }
            Day(18) => {
                println!("18:01 => {}", crate::day18::part01(&filename)?);
                println!("18:02 => {}", crate::day18::part02(&filename)?);
            }
            Day(19) => {
                println!("19:01 => {}", crate::day19::part01(&filename)?);
                println!("19:02 => {}", crate::day19::part02(&filename)?);
            }
            Day(20) => {
                println!("20:01 => {}", crate::day20::part01(&filename)?);
                println!("20:02 => {}", crate::day20::part02(&filename)?);
            }
            _ => println!("No solution for day {}", i32::from(&day)),
        }

        Ok(())
    }
}

fn generate_filename(day: &Day) -> PathBuf {
    PathBuf::from(format!("data/day{:02}.txt", i32::from(day)))
}
