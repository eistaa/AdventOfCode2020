use std::fs;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use std::collections::VecDeque;

fn parse(filename: &Path) -> Result<Vec<i64>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 09: {}", err))?
        .split_ascii_whitespace()
        .map(|v| i64::from_str(v).map_err(|err| format!("Failed to parse number: {}", err)))
        .collect()
}

fn find_trailing_sum(data: &Vec<i64>) -> Option<i64> {
    data.windows(26)
        .find(|window| {
            window[..25]
                .iter()
                .tuple_combinations()
                .all(|(first, last)| first + last != window[25])
        })
        .map(|window| window[25])
}

fn find_encryption_weakness(data: &Vec<i64>, target: i64) -> Option<i64> {
    let mut data = data.iter();
    let mut buffer: VecDeque<i64> = VecDeque::new();

    loop {
        let sum: i64 = buffer.iter().sum();
        if sum == target {
            break Some(*buffer.iter().min().unwrap() + *buffer.iter().max().unwrap());
        } else if sum > target {
            if buffer.is_empty() {
                break None;
            } else {
                buffer.pop_front();
            }
        } else {
            let next = data.next();
            if next.is_none() {
                break None;
            } else {
                buffer.push_back(*next.unwrap())
            }
        }
    }
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;
    match find_trailing_sum(&data) {
        Some(v) => Ok(v.to_string()),
        None => Err("Not found".to_string()),
    }
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;
    match find_encryption_weakness(&data, find_trailing_sum(&data).unwrap()) {
        Some(v) => Ok(v.to_string()),
        None => Err("Not found".to_string()),
    }
}
