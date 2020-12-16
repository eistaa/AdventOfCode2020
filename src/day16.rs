use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Rule {
    pub name: String,
    pub ranges: Vec<(i32, i32)>,
}

impl Rule {
    fn in_any_range(&self, value: &i32) -> bool {
        for (start, end) in &self.ranges {
            if *start <= *value && *value <= *end {
                return true;
            }
        }
        false
    }
}

fn parse(filename: &Path) -> Result<(Vec<Rule>, Vec<i32>, Vec<Vec<i32>>), String> {
    let re_rule = Regex::new(r"^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();

    let input_blocks: Vec<String> = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 15: {}", err))?
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    // rules
    let rules = input_blocks
        .get(0)
        .ok_or("No rules input block".to_string())?
        .lines()
        .map::<Result<Rule, String>, _>(|line| {
            let capture = re_rule.captures(line).ok_or("Failed to match rule line".to_string())?;
            Ok(Rule {
                name: capture.get(1).ok_or("No name on rule".to_string())?.as_str().to_owned(),
                ranges: capture
                    .iter()
                    .skip(2)
                    .map(|cap| i32::from_str(cap.unwrap().as_str()).unwrap())
                    .tuples()
                    .collect::<Vec<(i32, i32)>>(),
            })
        })
        .collect::<Result<Vec<Rule>, _>>()?;

    // my ticket
    let ticket = input_blocks
        .get(1)
        .ok_or("No my ticket input block".to_string())?
        .lines()
        .nth(1)
        .ok_or("Failed to retrieve my ticket line".to_string())?
        .split(",")
        .map(|num| i32::from_str(num).unwrap())
        .collect::<Vec<i32>>();

    // nearby tickets
    let tickets = input_blocks
        .get(2)
        .ok_or("No nearby tickets input block".to_string())?
        .lines()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|num| i32::from_str(num).unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Ok((rules, ticket, tickets))
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let (rules, _, tickets) = &parse(filename)?;

    let error_rate: i32 = tickets
        .iter()
        .flatten()
        .filter(|&num| !rules.iter().any(|rule| rule.in_any_range(num)))
        .sum();

    Ok(format!("Ticket error rate: {}", error_rate))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let (rules, ticket, tickets) = &parse(filename)?;

    let tickets = tickets
        .iter()
        .filter(|ticket| {
            !ticket
                .iter()
                .any(|field| !rules.iter().any(|rule| rule.in_any_range(field)))
        })
        .collect::<Vec<&Vec<i32>>>();

    let mut possible = Vec::new();
    for rule in rules {
        let mut fields = HashSet::new();
        for field in 0..rules.len() {
            if tickets.iter().all(|ticket| rule.in_any_range(&ticket[field])) {
                fields.insert(field);
            }
        }

        possible.push(fields);
    }

    let mut ordering = Vec::new();
    let mut found = HashSet::new();
    for (fields, idx) in possible.iter().zip(0..rules.len()).sorted_by_key(|el| el.0.len()) {
        ordering.push((idx, *fields.difference(&found).next().unwrap()));
        found.extend(fields);
    }

    Ok(format!(
        "Departure fields product: {}",
        ordering
            .iter()
            .filter(|(idx, _)| rules[*idx].name.starts_with("departure"))
            .map(|(_, field)| ticket[*field] as i64)
            .product::<i64>()
    ))
}
