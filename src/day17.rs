use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn parse(filename: &Path) -> Result<HashSet<(i8, i8, i8, i8)>, String> {
    Ok(fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 17: {}", err))?
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, cell)| cell == '#')
                .map(move |(x, _)| (x as i8, y as i8, 0, 0))
        })
        .flatten()
        .collect())
}

fn neighboring(origin: (i8, i8, i8, i8), three: bool) -> impl Iterator<Item = ((i8, i8, i8, i8), i8)> {
    (-1..2)
        .cartesian_product(-1..2)
        .cartesian_product(-1..2)
        .cartesian_product(if three { 0..1 } else { -1..2 })
        .map(move |(((x, y), z), w)| {
            (
                (origin.0 + x, origin.1 + y, origin.2 + z, origin.3 + w),
                if x != 0 || y != 0 || z != 0 || w != 0 { 1 } else { 0 },
            )
        })
}

fn step(grid: HashSet<(i8, i8, i8, i8)>, three: bool) -> HashSet<(i8, i8, i8, i8)> {
    let capacity = 81 * grid.len();
    grid.into_iter()
        .map(|coord| neighboring(coord, three))
        .flatten()
        .fold(
            HashMap::<_, (bool, i8), _>::with_capacity(capacity),
            |mut acc, (coord, count)| {
                acc.entry(coord)
                    .and_modify(|el| {
                        (*el).0 |= count == 0;
                        (*el).1 += count;
                    })
                    .or_insert((count == 0, count));
                acc
            },
        )
        .into_iter()
        .filter(|&(_, (active, count))| (!active && count == 3) || (active && (count == 2 || count == 3)))
        .map(|(coord, _)| coord)
        .collect()
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let mut cube = parse(filename)?;

    let steps = 6;
    for _ in 0..steps {
        cube = step(cube, true);
    }

    Ok(format!("Active cells: {}", cube.len()))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let mut cube = parse(filename)?;

    let steps = 6;
    for _ in 0..steps {
        cube = step(cube, false);
    }

    Ok(format!("Active cells: {}", cube.len()))
}
