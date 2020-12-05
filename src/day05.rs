use std::collections::BTreeSet;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;

fn parse_seat_code(code: &str) -> i32 {
    let row = i32::from_str_radix(&code[..7].replace("F", "0").replace("B", "1"), 2).unwrap();
    let col = i32::from_str_radix(&code[7..].replace("L", "0").replace("R", "1"), 2).unwrap();

    row * 8 + col
}

pub fn part01(filename: &Path) -> Result<String, String> {
    Ok(format!(
        "Maximum seat ID: {}",
        fs::read_to_string(filename)
            .map_err(|err| format!("Failed to read data for day 05: {}", err))?
            .split_ascii_whitespace()
            .map(parse_seat_code)
            .max()
            .unwrap()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let seats: BTreeSet<i32> = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 05: {}", err))?
        .split_ascii_whitespace()
        .map(parse_seat_code)
        .collect();

    let possible_seats: Vec<i32> = BTreeSet::from_iter(*seats.iter().min().unwrap()..*seats.iter().max().unwrap())
        .difference(&seats)
        .cloned()
        .collect();

    if possible_seats.len() == 1 {
        Ok(format!("Your seat ID: {}", possible_seats.first().unwrap()))
    } else if possible_seats.is_empty() {
        Err("There are no possible seats".to_string())
    } else {
        Err("There were multiple possible seats".to_string())
    }
}
