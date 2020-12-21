use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::fmt;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

const TILE_DIM: usize = 10;
const EMPTY_CELL: char = '.';
const SEA_CELL: char = '#';
const MONSTER_CELL: char = 'O';

#[derive(Copy, Clone, Debug)]
enum Transform {
    Flip,
    Rot,
    Pass,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
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

#[derive(Clone)]
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

    fn edge_top(&self) -> [Cell; TILE_DIM] {
        self.grid[0..TILE_DIM].try_into().unwrap()
    }
    fn edge_bottom(&self) -> [Cell; TILE_DIM] {
        self.grid[((TILE_DIM - 1) * TILE_DIM)..(TILE_DIM * TILE_DIM)]
            .try_into()
            .unwrap()
    }
    fn edge_left(&self) -> [Cell; TILE_DIM] {
        self.grid
            .iter()
            .step_by(TILE_DIM)
            .copied()
            .collect::<Vec<Cell>>()
            .try_into()
            .unwrap()
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
    fn edge(&self, dir: Direction) -> [Cell; TILE_DIM] {
        match dir {
            Direction::Top => self.edge_top(),
            Direction::Bottom => self.edge_bottom(),
            Direction::Left => self.edge_left(),
            Direction::Right => self.edge_right(),
        }
    }

    fn transform(&self, op: Transform) -> Self {
        let mut grid = [Cell::Empty; TILE_DIM * TILE_DIM];
        for y in 0..TILE_DIM {
            for x in 0..TILE_DIM {
                match op {
                    Transform::Flip => grid[TILE_DIM * y + x] = self.grid[TILE_DIM * y + (TILE_DIM - x - 1)],
                    Transform::Rot => grid[TILE_DIM * y + x] = self.grid[TILE_DIM * x + (TILE_DIM - y - 1)],
                    Transform::Pass => grid[TILE_DIM * y + x] = self.grid[TILE_DIM * y + x],
                }
            }
        }

        Tile { id: self.id, grid }
    }

    fn row_str(&self, row: usize) -> String {
        self.grid[(TILE_DIM * row)..(TILE_DIM * (row + 1))]
            .iter()
            .map(|cell| cell.to_string())
            .join("")
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

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
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
    pub corners: HashSet<u64>,
    pub edges: HashSet<u64>,
    pub interior: HashSet<u64>,
    pub tile_pairs: HashMap<u64, HashMap<[Cell; TILE_DIM], u64>>,
}

fn classify_tiles(tiles: &HashMap<u64, Tile>) -> Result<Classification, String> {
    let mut edge_counts: HashMap<_, HashSet<u64>> = HashMap::new();
    for tile in tiles.values() {
        // rotating twice reverses all edges
        for tile in &[tile, &tile.transform(Transform::Rot).transform(Transform::Rot)] {
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

    let mut tile_pairs: HashMap<_, HashMap<[Cell; TILE_DIM], u64>> = HashMap::new();
    for (edge, ids) in edge_counts.iter() {
        for &id in ids.iter() {
            tile_pairs
                .entry(id)
                .or_insert(HashMap::new())
                .extend(ids.iter().filter(|&&i| i != id).map(|&i| (*edge, i)));
        }
    }

    Ok(Classification {
        corners: edges
            .iter()
            .filter(|(_, &v)| v == 4) // 2 normal and 2 flipped edges
            .map(|(&k, _)| k)
            .collect(),
        edges: edges
            .iter()
            .filter(|(_, &v)| v == 2) // 1 normal and 1 flipped edge
            .map(|(&k, _)| k)
            .collect(),
        interior: tiles
            .iter()
            .filter(|(k, _)| !edges.contains_key(k))
            .map(|(&k, _)| k)
            .collect(),
        tile_pairs,
    })
}

fn assemble(tiles: &HashMap<u64, Tile>, data: &Classification) -> Vec<String> {
    struct Carrier {
        tile: Tile,
        up: Option<u64>,
        down: Option<u64>,
        left: Option<u64>,
        right: Option<u64>,
    }

    impl Carrier {
        fn is_fixed(&self) -> bool {
            self.up.is_some() || self.down.is_some() || self.left.is_some() || self.right.is_some()
        }
    }

    fn orient(dir: Direction, edge: &[Cell; TILE_DIM], mut tile: Tile) -> Option<Tile> {
        use Transform::*;
        for &op in &[Pass, Flip] {
            tile = tile.transform(op);
            for &op in &[Pass, Rot, Rot, Rot] {
                tile = tile.transform(op);
                if &tile.edge(dir) == edge {
                    return Some(tile);
                }
            }
        }

        None
    }

    let mut fixed: HashMap<u64, Carrier> = HashMap::new();
    let mut queue = VecDeque::from_iter(tiles.keys().copied());

    // find assembly of tiles
    while !queue.is_empty() {
        let mut carrier = Carrier {
            tile: tiles.get(&queue.pop_front().unwrap()).unwrap().clone(),
            up: None,
            down: None,
            left: None,
            right: None,
        };

        if fixed.is_empty() {
            fixed.insert(carrier.tile.id, carrier);
            continue;
        }

        for (edge, neighbor_id) in data.tile_pairs.get(&carrier.tile.id).unwrap().iter() {
            // have we encountered the neighbor?
            if let Some(neighbor) = fixed.get_mut(neighbor_id) {
                // determine if we have the correctly oriented edge
                if neighbor.up.is_none() && &neighbor.tile.edge_top() == edge {
                    neighbor.up = Some(carrier.tile.id);
                    if !carrier.is_fixed() {
                        carrier.tile = orient(Direction::Bottom, edge, carrier.tile.clone())
                            .expect("Failed to orient matched tile");
                    }
                    carrier.down = Some(*neighbor_id);
                } else if neighbor.down.is_none() && &neighbor.tile.edge_bottom() == edge {
                    neighbor.down = Some(carrier.tile.id);
                    if !carrier.is_fixed() {
                        carrier.tile =
                            orient(Direction::Top, edge, carrier.tile.clone()).expect("Failed to orient matched tile");
                    }
                    carrier.up = Some(*neighbor_id);
                } else if neighbor.left.is_none() && &neighbor.tile.edge_left() == edge {
                    neighbor.left = Some(carrier.tile.id);
                    if !carrier.is_fixed() {
                        carrier.tile = orient(Direction::Right, edge, carrier.tile.clone())
                            .expect("Failed to orient matched tile");
                    }
                    carrier.right = Some(*neighbor_id);
                } else if neighbor.right.is_none() && &neighbor.tile.edge_right() == edge {
                    neighbor.right = Some(carrier.tile.id);
                    if !carrier.is_fixed() {
                        carrier.tile =
                            orient(Direction::Left, edge, carrier.tile.clone()).expect("Failed to orient matched tile");
                    }
                    carrier.left = Some(*neighbor_id);
                }
            }
        }

        if carrier.is_fixed() {
            fixed.insert(carrier.tile.id, carrier);
        } else {
            queue.push_back(carrier.tile.id);
        }
    }

    let mut assembly = Vec::new();
    let mut tile_id = data
        .corners
        .iter()
        .map(|id| fixed.get(id).unwrap())
        .filter(|c| c.up.is_none() && c.left.is_none())
        .map(|c| c.tile.id)
        .next();
    while tile_id.is_some() {
        let leftmost_id = tile_id;
        for row in 1..(TILE_DIM - 1) {
            assembly.push(String::new());
            while tile_id.is_some() {
                let tile = fixed.get(&tile_id.unwrap()).unwrap();
                assembly
                    .last_mut()
                    .unwrap()
                    .push_str(&tile.tile.row_str(row)[1..(TILE_DIM - 1)]);
                tile_id = tile.right;
            }
            tile_id = leftmost_id;
        }
        tile_id = fixed.get(&tile_id.unwrap()).unwrap().down;
    }

    assembly
}

fn scan_for_monsters(mut assembly: Vec<String>) -> String {
    let re_1 = Regex::new(r"(.{18})#(.)").unwrap();
    let re_2 = Regex::new(r"#(.{4})##(.{4})##(.{4})###").unwrap();
    let re_3 = Regex::new(r"(.)#(.{2})#(.{2})#(.{2})#(.{2})#(.{2})#(.{3})").unwrap();

    let mut with_monsters = Vec::new();
    let mut matched = false;
    while !matched {
        with_monsters.clear();
        let mut line_1 = assembly.get(0).unwrap().clone();
        let mut line_2 = assembly.get(1).unwrap().clone();

        for row in 2..assembly.len() {
            let mut line_3 = assembly.get(row).unwrap().clone();
            if let Some(res_2) = re_2.find(&line_2) {
                let res_1 = re_1.find_at(&line_1, res_2.start());
                let res_3 = re_3.find_at(&line_3, res_2.start());

                if res_1.is_some()
                    && res_1.unwrap().start() == res_2.start()
                    && res_3.is_some()
                    && res_3.unwrap().start() == res_2.start()
                {
                    let range = res_2.start()..res_2.end();
                    line_1.replace_range(range.clone(), re_1.replace(&line_1[range.clone()].to_owned(), "${1}O${2}").as_ref());
                    line_2.replace_range(range.clone(), re_2.replace(&line_2[range.clone()].to_owned(), "O${1}OO${2}OO${3}OOO").as_ref());
                    line_3.replace_range(range.clone(), re_3.replace(&line_3[range.clone()].to_owned(), "${1}O${2}O${3}O${4}O${5}O${6}O${7}").as_ref());

                    matched = true;
                }
            }

            with_monsters.push(line_1);
            line_1 = line_2;
            line_2 = line_3;
        }
        with_monsters.push(line_1);
        with_monsters.push(line_2);

        assembly = rotate(assembly);
    }

    with_monsters.join("\n")
}

fn rotate(assembly: Vec<String>) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in assembly.into_iter() {
        for (i, c) in line.chars().rev().enumerate() {
            if let Some(new_line) = lines.get_mut(i) {
                new_line.push(c);
            } else {
                lines.push(c.to_string());
            }
        }
    }

    lines
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let tiles = parse(filename)?;
    let classification = classify_tiles(&tiles)?;

    Ok(format!(
        "Product of corner tile ids: {}",
        classification.corners.iter().product::<u64>()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let tiles = parse(filename)?;
    let classification = classify_tiles(&tiles)?;

    let assembly = scan_for_monsters(assemble(&tiles, &classification));
    println!("{}", assembly);

    Ok(format!("{}", assembly.chars().filter(|&c| c == '#').count()))
}
