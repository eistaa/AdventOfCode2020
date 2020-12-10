use std::fs;
use std::path::Path;
use std::str::FromStr;

fn parse(filename: &Path) -> Result<Vec<usize>, String> {
    let mut numbers = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 10: {}", err))?
        .split_ascii_whitespace()
        .map(|v| usize::from_str(v).map_err(|err| format!("Failed to parse number: {}", err)))
        .collect::<Result<Vec<usize>, _>>()?;

    numbers.sort();
    Ok(numbers)
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    // one and three diff, and previous joltages
    let mut d1 = 0;
    let mut d3 = 1; // laptop is 3 higher than it's charger
    let mut pj = 0; // plane adaptor is zero

    for j in data.iter() {
        if j - pj == 1 {
            d1 += 1;
        } else if j - pj == 3 {
            d3 += 1;
        }
        pj = *j;
    }

    Ok(format!("Product of 1-diff and 3-diff joltages: {}", d1 * d3))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    // previous 3, joltage and combinations so far
    let mut p3: (usize, usize) = (0, 0);
    let mut p2: (usize, usize) = (0, 0);
    let mut p1: (usize, usize) = (0, 1); // represents connecting to the outlet

    for i in data.iter() {
        // compute possible chargers the current can connect to
        let v = if i - p3.0 <= 3 {
            p3.1 + p2.1 + p1.1
        } else if i - p2.0 <= 3 {
            p2.1 + p1.1
        } else if i - p1.0 <= 3 {
            p1.1
        } else {
            0
        };

        p3 = p2;
        p2 = p1;
        p1 = (*i, v);
    }

    Ok(format!("Total distinct charger combinations: {}", p1.1))
}
