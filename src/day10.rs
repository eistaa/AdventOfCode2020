use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn parse(filename: &Path) -> Result<Vec<i32>, String> {
    let mut numbers = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 10: {}", err))?
        .split_ascii_whitespace()
        .map(|v| i32::from_str(v).map_err(|err| format!("Failed to parse number: {}", err)))
        .collect::<Result<Vec<i32>, _>>()?;

    numbers.sort();
    Ok(numbers)
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    let mut j1 = 0;
    let mut j3 = 1; // laptop 3 higher than its charger
    let mut pj = 0; // plane adaptor is zero
    for j in data.iter() {
        if j - pj == 1 {
            j1 += 1;
        } else if j - pj == 3 {
            j3 += 1;
        }
        pj = *j;
    }

    Ok(format!("Product of 1-diff and 3-dif joltages: {}", j1 * j3))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    let mut ways = HashMap::<i32, i64, _>::new();
    ways.insert(0, 1);
    for i in data.iter() {
        ways.insert(
            *i,
            ways.get(&(i - 1)).unwrap_or(&0) + ways.get(&(i - 2)).unwrap_or(&0) + ways.get(&(i - 3)).unwrap_or(&0),
        );
    }

    Ok(format!(
        "Total distinct charger combinations: {}",
        ways.get(ways.keys().max().unwrap()).unwrap()
    ))
}
