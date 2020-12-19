use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Rulespec {
    Literal(char),
    Single(u8),
    Chain(u8, u8),
    Either(u8, u8),
    EitherChain((u8, u8), (u8, u8)),
}

impl Rulespec {
    fn new(spec: &str) -> Result<Self, String> {
        let re_lit = Regex::new(r"^\x22([a-z])\x22$").unwrap();
        let re_single = Regex::new(r"^(\d+)$").unwrap();
        let re_chain = Regex::new(r"^(\d+) (\d+)$").unwrap();
        let re_either = Regex::new(r"^(\d+) \| (\d+)$").unwrap();
        let re_either_chain = Regex::new(r"^(\d+) (\d+) \| (\d+) (\d+)$").unwrap();

        Ok(if let Some(cap) = re_lit.captures(spec) {
            Self::Literal(cap.get(1).unwrap().as_str().chars().next().unwrap())
        } else if let Some(cap) = re_single.captures(spec) {
            Self::Single(
                u8::from_str(cap.get(1).unwrap().as_str())
                    .map_err(|err| format!("Failed to parse single rule number: {}", err))?,
            )
        } else if let Some(cap) = re_chain.captures(spec) {
            Self::Chain(
                u8::from_str(cap.get(1).unwrap().as_str())
                    .map_err(|err| format!("Failed to parse chain rule number: {}", err))?,
                u8::from_str(cap.get(2).unwrap().as_str())
                    .map_err(|err| format!("Failed to parse chain rule number: {}", err))?,
            )
        } else if let Some(cap) = re_either.captures(spec) {
            Self::Either(
                u8::from_str(cap.get(1).unwrap().as_str())
                    .map_err(|err| format!("Failed to parse either rule number: {}", err))?,
                u8::from_str(cap.get(2).unwrap().as_str())
                    .map_err(|err| format!("Failed to parse either rule number: {}", err))?,
            )
        } else if let Some(cap) = re_either_chain.captures(spec) {
            Self::EitherChain(
                (
                    u8::from_str(cap.get(1).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse either chain rule number: {}", err))?,
                    u8::from_str(cap.get(2).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse either chain rule number: {}", err))?,
                ),
                (
                    u8::from_str(cap.get(3).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse either chain rule number: {}", err))?,
                    u8::from_str(cap.get(4).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse either chain rule number: {}", err))?,
                ),
            )
        } else {
            Err(format!("Rulespec does not match any rule: {}", spec))?
        })
    }
}

fn parse(filename: &Path) -> Result<(HashMap<u8, Rulespec>, Vec<String>), String> {
    let blocks: Vec<String> = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 19: {}", err))?
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    Ok((
        blocks[0]
            .lines()
            .map(|line| {
                let ruleparts: Vec<&str> = line.split(": ").collect();
                Ok((
                    u8::from_str(ruleparts[0]).map_err(|err| format!("Failed to parse rule number: {}", err))?,
                    Rulespec::new(ruleparts[1])?,
                ))
            })
            .collect::<Result<HashMap<u8, Rulespec>, String>>()?,
        blocks[1].lines().map(|s| s.to_string()).collect(),
    ))
}

fn rulemap_to_re(start: u8, map: &HashMap<u8, Rulespec>) -> Result<String, String> {
    let mut stack = vec![(start, map.get(&start).ok_or(format!("Missing start rule {}", start))?)];
    let mut processed = HashMap::new();

    while !stack.is_empty() {
        match *stack.last().unwrap() {
            (id, Rulespec::Literal(c)) => {
                stack.pop();
                processed.insert(id, c.to_string());
            }
            (id, Rulespec::Single(r1)) => {
                if !processed.contains_key(&r1) {
                    stack.push((*r1, map.get(r1).ok_or(format!("Missing rule {}", r1))?))
                } else {
                    stack.pop();
                    processed.insert(id, format!("{}", processed.get(r1).unwrap()));
                }
            }
            (id, Rulespec::Chain(r1, r2)) => {
                if !processed.contains_key(&r1) || !processed.contains_key(&r2) {
                    if !processed.contains_key(&r1) {
                        stack.push((*r1, map.get(r1).ok_or(format!("Missing rule {}", r1))?))
                    }
                    if !processed.contains_key(&r2) {
                        stack.push((*r2, map.get(r2).ok_or(format!("Missing rule {}", r2))?))
                    }
                } else {
                    stack.pop();
                    processed.insert(
                        id,
                        format!("{}{}", processed.get(r1).unwrap(), processed.get(r2).unwrap()),
                    );
                }
            }
            (id, Rulespec::Either(r1, r2)) => {
                if !processed.contains_key(&r1) || !processed.contains_key(&r2) {
                    if !processed.contains_key(&r1) {
                        stack.push((*r1, map.get(r1).ok_or(format!("Missing rule {}", r1))?))
                    }
                    if !processed.contains_key(&r2) {
                        stack.push((*r2, map.get(r2).ok_or(format!("Missing rule {}", r2))?))
                    }
                } else {
                    stack.pop();
                    processed.insert(
                        id,
                        format!("({}|{})", processed.get(r1).unwrap(), processed.get(r2).unwrap()),
                    );
                }
            }
            (id, Rulespec::EitherChain((r11, r12), (r21, r22))) => {
                if !processed.contains_key(&r11)
                    || !processed.contains_key(&r12)
                    || !processed.contains_key(&r21)
                    || !processed.contains_key(&r22)
                {
                    if !processed.contains_key(&r11) {
                        stack.push((*r11, map.get(r11).ok_or(format!("Missing rule {}", r11))?))
                    }
                    if !processed.contains_key(&r12) {
                        stack.push((*r12, map.get(r12).ok_or(format!("Missing rule {}", r12))?))
                    }
                    if !processed.contains_key(&r21) {
                        stack.push((*r21, map.get(r21).ok_or(format!("Missing rule {}", r21))?))
                    }
                    if !processed.contains_key(&r22) {
                        stack.push((*r22, map.get(r22).ok_or(format!("Missing rule {}", r22))?))
                    }
                } else {
                    stack.pop();
                    processed.insert(
                        id,
                        format!(
                            "({}{}|{}{})",
                            processed.get(r11).unwrap(),
                            processed.get(r12).unwrap(),
                            processed.get(r21).unwrap(),
                            processed.get(r22).unwrap()
                        ),
                    );
                }
            }
        }
    }

    Ok(format!("^{}$", processed.get(&start).unwrap().to_owned()))
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let (rulemap, text) = parse(filename)?;
    let re = Regex::new(&rulemap_to_re(0, &rulemap)?).map_err(|err| format!("Generated invalid regex: {}", err))?;

    Ok(format!(
        "Matching lines: {}",
        text.iter().filter(|line| re.is_match(line)).count()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let (mut rulemap, text) = parse(filename)?;

    Ok(format!(""))
}
