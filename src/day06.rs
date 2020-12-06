use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn part01(filename: &Path) -> Result<String, String> {
    let yes_answers: usize = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 06: {}", err))?
        .split("\n\n")
        .map(|answers| {
            answers
                .lines()
                .map(|line| line.chars())
                .flatten()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();

    Ok(format!("Sum of any YES answers: {}", yes_answers))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let yes_answers: usize = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 06: {}", err))?
        .split("\n\n")
        .map(|answers| {
            answers
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .fold::<Option<HashSet<char>>, _>(None, |acc, other| {
                    Some(match &acc {
                        Some(map) => map.intersection(&other).cloned().collect(),
                        None => other,
                    })
                })
                .unwrap()
                .len()
        })
        .sum();

    Ok(format!("Sum of all YES answers: {}", yes_answers))
}
