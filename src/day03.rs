use std::fs;
use std::path::Path;
use std::convert::TryFrom;

const OPEN: char = '.';
const TREE: char = '#';

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Feature {
    Open,
    Tree,
}

impl TryFrom<char> for Feature {
    type Error = String;

    fn try_from(c: char) -> Result<Feature, String> {
        match c {
            OPEN => Ok(Feature::Open),
            TREE => Ok(Feature::Tree),
            _ => Err(format!("Unknown feature spec: {}", c))
        }
    }
}

fn parse(filename: &Path) -> Result<Vec<Vec<Feature>>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 03: {}", err))?
        .split_ascii_whitespace()
        .map(|line| {
            line.chars()
                .map(Feature::try_from)
                .collect::<Result<Vec<Feature>, String>>()
        })
        .collect::<Result<Vec<Vec<Feature>>, _>>()
}

fn get_feature(landscape: &Vec<Vec<Feature>>, y: usize, x: usize) -> Result<Feature, String> {
    let row_len = landscape[0].len();
    Ok(*landscape
        .get(y)
        .ok_or(&format!("No such landscape row: {}", y))?
        .get(x % row_len)
        .ok_or(&format!("No such landscape column: {} ({})", x % row_len, x))?)
}

fn angled_path_trees(landscape: &Vec<Vec<Feature>>, dy: usize, dx: usize) -> Result<usize, String> {
    Ok((0..landscape.len())
        .step_by(dy)
        .enumerate()
        .map(|(col, row)| get_feature(&landscape, row, col * dx))
        .collect::<Result<Vec<Feature>, String>>()?
        .iter()
        .filter(|feat| *feat == &Feature::Tree)
        .count())
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let landscape = parse(filename)?;
    Ok(format!("Trees in path: {}", angled_path_trees(&landscape, 1, 3)?))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let landscape = parse(filename)?;

    let first = angled_path_trees(&landscape, 1, 1)?;
    let second = angled_path_trees(&landscape, 1, 3)?;
    let third = angled_path_trees(&landscape, 1, 5)?;
    let fourth = angled_path_trees(&landscape, 1, 7)?;
    let fifth = angled_path_trees(&landscape, 2, 1)?;

    Ok(format!(
        "Trees in path product: {} * {} * {} * {} * {} = {}",
        first,
        second,
        third,
        fourth,
        fifth,
        first * second * third * fourth * fifth
    ))
}
