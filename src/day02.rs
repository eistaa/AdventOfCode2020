use std::fs;
use std::path::Path;
use std::str::FromStr;

use regex::Regex;

#[derive(Clone, Debug)]
struct PasswordData {
    lower: i32,
    upper: i32,
    char: char,
    password: String,
}

fn parse(filename: &Path) -> Result<Vec<PasswordData>, String> {
    let pattern = Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+([a-z]+)$").unwrap();

    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 02: {}", err))?
        .split("\n")
        .map(|line| {
            let captures = pattern.captures(line).ok_or("Failed to parse line in day 2 data")?;

            Ok(PasswordData {
                lower: i32::from_str(captures.get(1).unwrap().as_str())
                    .map_err(|err| format!("Invalid lower bound: {}", err))?,
                upper: i32::from_str(captures.get(2).unwrap().as_str())
                    .map_err(|err| format!("Invalid upper bound: {}", err))?,
                char: captures.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                password: captures.get(4).unwrap().as_str().to_string(),
            })
        })
        .collect::<Result<Vec<PasswordData>, _>>()
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let password_data = parse(filename)?;

    let valid_passwords = password_data
        .iter()
        .filter(|data| {
            let len: i32 = data
                .password
                .chars()
                .into_iter()
                .filter(|c| c == &data.char)
                .collect::<String>()
                .len() as i32;
            data.lower <= len && len <= data.upper
        })
        .cloned()
        .collect::<Vec<_>>()
        .len();

    Ok(format!(
        "Valid passwords: {} (total: {})",
        valid_passwords,
        password_data.len()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let password_data = parse(filename)?;

    let valid_passwords = password_data
        .iter()
        .filter(|data| {
            let first = data.password.chars().nth((data.lower - 1) as usize) == Some(data.char);
            let second = data.password.chars().nth((data.upper - 1) as usize) == Some(data.char);

            (first || second) && !(first && second)
        })
        .cloned()
        .collect::<Vec<_>>()
        .len();

    Ok(format!(
        "Valid passwords: {} (total: {})",
        valid_passwords,
        password_data.len()
    ))
}
