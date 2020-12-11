use std::convert::TryFrom;
use std::fs;
use std::path::Path;

const FLOOR: char = '.';
const EMPTY: char = 'L';
const TAKEN: char = '#';

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Taken,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Cell, String> {
        match c {
            FLOOR => Ok(Cell::Floor),
            EMPTY => Ok(Cell::Empty),
            TAKEN => Ok(Cell::Taken),
            _ => Err(format!("Unknown cell spec: {}", c)),
        }
    }
}

fn parse(filename: &Path) -> Result<Vec<Vec<Cell>>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 11: {}", err))?
        .lines()
        .map(|line| line.chars().map(Cell::try_from).collect::<Result<Vec<Cell>, _>>())
        .collect::<Result<Vec<Vec<Cell>>, _>>()
}

fn adjacent_pt1(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> [Option<Cell>; 8] {
    [
        if y > 0 && x > 0 {
            grid.get(y - 1).map(|row| row.get(x - 1)).flatten().copied()
        } else {
            None
        },
        if y > 0 {
            grid.get(y - 1).map(|row| row.get(x)).flatten().copied()
        } else {
            None
        },
        if y > 0 {
            grid.get(y - 1).map(|row| row.get(x + 1)).flatten().copied()
        } else {
            None
        },
        if x > 0 {
            grid.get(y).map(|row| row.get(x - 1)).flatten().copied()
        } else {
            None
        },
        grid.get(y).map(|row| row.get(x + 1)).flatten().copied(),
        if x > 0 {
            grid.get(y + 1).map(|row| row.get(x - 1)).flatten().copied()
        } else {
            None
        },
        grid.get(y + 1).map(|row| row.get(x)).flatten().copied(),
        grid.get(y + 1).map(|row| row.get(x + 1)).flatten().copied(),
    ]
}

fn scan_grid(grid: &Vec<Vec<Cell>>, delta: (isize, isize), origin: (usize, usize)) -> Option<Cell> {
    let mut x = origin.0 as isize;
    let mut y = origin.1 as isize;

    loop {
        if (delta.0 == 0 || x + delta.0 >= 0) && (delta.1 == 0 || y + delta.1 >= 0) {
            x = x + delta.0;
            y = y + delta.1;
            let cell = grid.get(y as usize)?.get(x as usize)?;
            match cell {
                Cell::Floor => continue,
                Cell::Taken | Cell::Empty => break Some(*cell),
            }
        } else {
            break None;
        }
    }
}

fn adjacent_pt2(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> [Option<Cell>; 8] {
    [
        scan_grid(grid, (-1, -1), (x, y)),
        scan_grid(grid, (-1, 0), (x, y)),
        scan_grid(grid, (-1, 1), (x, y)),
        scan_grid(grid, (0, -1), (x, y)),
        scan_grid(grid, (0, 1), (x, y)),
        scan_grid(grid, (1, -1), (x, y)),
        scan_grid(grid, (1, 0), (x, y)),
        scan_grid(grid, (1, 1), (x, y)),
    ]
}

fn cell_rule(cell: &Cell, adjacent: &[Option<Cell>; 8], taken_count: usize) -> Cell {
    match cell {
        Cell::Floor => *cell,
        Cell::Empty => {
            if !adjacent.contains(&Some(Cell::Taken)) {
                Cell::Taken
            } else {
                *cell
            }
        }
        Cell::Taken => {
            if adjacent.iter().filter(|&&c| c == Some(Cell::Taken)).count() >= taken_count {
                Cell::Empty
            } else {
                *cell
            }
        }
    }
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let mut grid = parse(filename)?;
    let height = grid.len();
    let width = grid[0].len();

    loop {
        let mut next = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(cell_rule(&grid[y][x], &adjacent_pt1(&grid, x, y), 4));
            }
            next.push(row);
        }
        if next == grid {
            break;
        } else {
            grid = next;
        }
    }

    Ok(format!(
        "Seats taken when stabilized: {}",
        grid.iter()
            .map(|row| row.iter().filter(|&&cell| cell == Cell::Taken).count())
            .sum::<usize>()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let mut grid = parse(filename)?;
    let height = grid.len();
    let width = grid[0].len();

    loop {
        let mut next = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                row.push(cell_rule(&grid[y][x], &adjacent_pt2(&grid, x, y), 5));
            }
            next.push(row);
        }
        if next == grid {
            break;
        } else {
            grid = next;
        }
    }

    Ok(format!(
        "Seats taken when stabilized: {}",
        grid.iter()
            .map(|row| row.iter().filter(|&&cell| cell == Cell::Taken).count())
            .sum::<usize>()
    ))
}
