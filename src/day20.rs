use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::iter::FromIterator;

const TILE_DIM: usize = 10;
const EMPTY_CELL: char = '.';
const SEA_CELL: char = '#';
const MONSTER_CELL: char = 'O';

#[derive(Copy, Clone, Debug)]
enum Transform {
    Flip,
    Rot,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Sea,
    Monster,
}

impl Cell {
    fn try_new(c: char) -> Result<Self, String> {
        match c {
            EMPTY_CELL => Ok(Cell::Empty),
            SEA_CELL => Ok(Cell::Sea),
            MONSTER_CELL => Ok(Cell::Monster),
            _ => Err(format!("Unknown cell spec: {}", c)),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => EMPTY_CELL,
                Self::Sea => SEA_CELL,
                Self::Monster => MONSTER_CELL,
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Tile {
    pub grid: [Cell; TILE_DIM * TILE_DIM],
    pub id: u64,
}

impl Tile {
    fn try_new(id: u64, lines: &Vec<&str>) -> Result<Self, String> {
        if lines.len() != TILE_DIM {
            Err(format!("Only {0}x{0} tiles allowed: wrong number of rows", TILE_DIM))?
        }

        let mut grid = [Cell::Empty; TILE_DIM * TILE_DIM];
        for (y, &line) in lines.iter().enumerate() {
            if line.len() != TILE_DIM {
                Err(format!("Only {0}x{0} tiles allowed: wrong number of columns", TILE_DIM))?
            }
            for (x, c) in line.chars().enumerate() {
                grid[TILE_DIM * y + x] = Cell::try_new(c)?;
            }
        }

        Ok(Tile { id, grid })
    }

    // edges are directed clockwise

    fn edge_top(&self) -> [Cell; TILE_DIM] {
        self.grid[0..TILE_DIM].try_into().unwrap()
    }
    fn edge_bottom(&self) -> [Cell; TILE_DIM] {
        let mut edge: [Cell; TILE_DIM] = self.grid[((TILE_DIM - 1) * TILE_DIM)..(TILE_DIM * TILE_DIM)]
            .try_into()
            .unwrap();
        edge.reverse();
        edge
    }
    fn edge_left(&self) -> [Cell; TILE_DIM] {
        let mut edge: [Cell; TILE_DIM] = self
            .grid
            .iter()
            .step_by(TILE_DIM)
            .copied()
            .collect::<Vec<Cell>>()
            .try_into()
            .unwrap();
        edge.reverse();
        edge
    }
    fn edge_right(&self) -> [Cell; TILE_DIM] {
        self.grid
            .iter()
            .skip(TILE_DIM - 1)
            .step_by(TILE_DIM)
            .copied()
            .collect::<Vec<Cell>>()
            .try_into()
            .unwrap()
    }

    fn transform(&self, op: Transform) -> Self {
        let mut grid = [Cell::Empty; TILE_DIM * TILE_DIM];
        for y in 0..TILE_DIM {
            for x in 0..TILE_DIM {
                match op {
                    Transform::Flip => grid[TILE_DIM * y + x] = self.grid[TILE_DIM * y + (TILE_DIM - x - 1)],
                    Transform::Rot => grid[TILE_DIM * y + x] = self.grid[TILE_DIM * x + (TILE_DIM - y - 1)],
                }
            }
        }

        Tile { id: self.id, grid }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            (0..TILE_DIM)
                .map(|y| {
                    self.grid[(TILE_DIM * y)..(TILE_DIM * (y + 1))]
                        .iter()
                        .map(|cell| cell.to_string())
                        .collect::<String>()
                })
                .join("\n")
        )
    }
}

fn parse(filename: &Path) -> Result<HashMap<u64, Tile>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 20: {}", err))?
        .trim()
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let id = u64::from_str(&lines.next().ok_or("No id line for tile".to_string())?[5..9])
                .map_err(|err| format!("Failed to parse tile id: {}", err))?;
            Ok((id, Tile::try_new(id, &lines.collect())?))
        })
        .collect::<Result<HashMap<u64, Tile>, _>>()
}

#[derive(Clone, Debug)]
struct Classification {
    pub corners: HashMap<u64, Tile>,
    pub edges: HashMap<u64, Tile>,
    pub interior: HashMap<u64, Tile>,
    pub tile_pairs: HashMap<u64, HashSet<u64>>,
}

fn classify_tiles(tiles: &HashMap<u64, Tile>) -> Result<Classification, String> {
    let mut edge_counts: HashMap<_, HashSet<u64>> = HashMap::new();
    for tile in tiles.values() {
        for tile in &[tile, &tile.transform(Transform::Flip)] {
            edge_counts
                .entry(tile.edge_top())
                .or_insert(HashSet::new())
                .insert(tile.id);
            edge_counts
                .entry(tile.edge_bottom())
                .or_insert(HashSet::new())
                .insert(tile.id);
            edge_counts
                .entry(tile.edge_left())
                .or_insert(HashSet::new())
                .insert(tile.id);
            edge_counts
                .entry(tile.edge_right())
                .or_insert(HashSet::new())
                .insert(tile.id);
        }
    }

    let mut edges = HashMap::new();
    for id in edge_counts
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(_, v)| *v.iter().next().unwrap())
    {
        edges.entry(id).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut tile_pairs: HashMap<_, HashSet<u64>> = HashMap::new();
    for ids in edge_counts.values() {
        for &id in ids.iter() {
            tile_pairs.entry(id).or_insert(HashSet::new()).extend(ids.iter().filter(|&&i| i != id));
        }
    }

    Ok(Classification {
        corners: edges
            .iter()
            .filter(|(_, &v)| v == 4) // 2 normal and 2 flipped edges
            .map(|(&k, _)| (k, tiles.get(&k).unwrap().clone()))
            .collect(),
        edges: edges
            .iter()
            .filter(|(_, &v)| v == 2) // 1 normal and 1 flipped edge
            .map(|(&k, _)| (k, tiles.get(&k).unwrap().clone()))
            .collect(),
        interior: tiles
            .iter()
            .filter(|(k, _)| !edges.contains_key(k))
            .map(|(&k, _)| (k, tiles.get(&k).unwrap().clone()))
            .collect(),
        tile_pairs,
    })
}

fn assemble(data: &Classification) -> () {

}

pub fn part01(filename: &Path) -> Result<String, String> {
    let tiles = parse(filename)?;
    let classification = classify_tiles(&tiles)?;

    Ok(format!(
        "Product of corner tile ids: {}",
        classification.corners.keys().product::<u64>()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let tiles = parse(filename)?;
    let classification = classify_tiles(&tiles)?;

    dbg!(classification.tile_pairs);

    Ok(format!(""))
}
