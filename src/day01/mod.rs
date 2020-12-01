use std::fs;
use std::path::Path;
use std::str::FromStr;

fn parse(filename: &Path) -> Result<Vec<i32>, String> {
    let mut numbers: Result<Vec<i32>, _> = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 01: {}", err))?
        .split_ascii_whitespace()
        .map(|token| {
            i32::from_str(token).map_err(|err| format!("Invalid data in file for day 01: {}", err))
        })
        .collect();

    if let Ok(nums) = &mut numbers {
        nums.sort();
    }

    numbers
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let numbers = parse(filename)?;

    for i in 0..numbers.len() {
        let first = &numbers[i];
        for j in ((i + 1)..numbers.len()).rev() {
            let second = &numbers[j];
            if first + second > 2020 {
                continue;
            } else if first + second == 2020 {
                return Ok(format!("{} * {} = {}", first, second, first * second));
            } else {
                break;
            }
        }
    }

    Err(format!(
        "{:?}",
        "Failed to find solution for day 01, part 1..."
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let numbers = parse(filename)?;

    for i in 0..numbers.len() {
        let first = &numbers[i];
        for j in ((i + 1)..numbers.len()).rev() {
            let second = &numbers[j];
            if first + second >= 2020 {
                continue;
            } else {
                for k in 0..numbers.len() {
                    let third = &numbers[k];
                    if first + second + third < 2020 {
                        continue;
                    } else if first + second + third == 2020 {
                        return Ok(format!(
                            "{} * {} * {} = {}",
                            first,
                            second,
                            third,
                            first * second * third
                        ));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    Err(format!(
        "{:?}",
        "Failed to find solution for day 01, part 2..."
    ))
}
