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

impl Default for Cell {
    fn default() -> Self {
        Cell::Floor
    }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(p: (isize, isize)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    cells: Vec<Cell>,
    pub height: usize,
    pub width: usize,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        let mut cells = Vec::with_capacity(height * width);
        cells.resize_with(height * width, Default::default);

        Grid { cells, height, width }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells.get(y * self.width + x)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.cells.get_mut(y * self.width + x)
        }
    }

    pub fn adjacent_next(&self, origin: &Point) -> [Option<&Cell>; 8] {
        [
            self.get_offset(origin, &(-1, -1).into()),
            self.get_offset(origin, &(-1, 0).into()),
            self.get_offset(origin, &(-1, 1).into()),
            self.get_offset(origin, &(0, -1).into()),
            self.get_offset(origin, &(0, 1).into()),
            self.get_offset(origin, &(1, -1).into()),
            self.get_offset(origin, &(1, 0).into()),
            self.get_offset(origin, &(1, 1).into()),
        ]
    }

    pub fn adjacent_nearest<F>(&self, origin: &Point, op: F) -> [Option<&Cell>; 8]
    where
        F: Fn(&Cell) -> bool,
    {
        [
            self.scan_delta(origin, &(-1, -1).into(), &op),
            self.scan_delta(origin, &(-1, 0).into(), &op),
            self.scan_delta(origin, &(-1, 1).into(), &op),
            self.scan_delta(origin, &(0, -1).into(), &op),
            self.scan_delta(origin, &(0, 1).into(), &op),
            self.scan_delta(origin, &(1, -1).into(), &op),
            self.scan_delta(origin, &(1, 0).into(), &op),
            self.scan_delta(origin, &(1, 1).into(), &op),
        ]
    }

    pub fn get_offset(&self, origin: &Point, offset: &Point) -> Option<&Cell> {
        let x = origin.x + offset.x;
        let y = origin.y + offset.y;

        if x >= 0 && y >= 0 {
            self.get(x as usize, y as usize)
        } else {
            None
        }
    }

    pub fn scan_delta<F>(&self, origin: &Point, delta: &Point, op: &F) -> Option<&Cell>
    where
        F: Fn(&Cell) -> bool,
    {
        let mut x = origin.x;
        let mut y = origin.y;

        if delta.x != 0 || delta.y != 0 {
            loop {
                if (delta.x == 0 || x + delta.x >= 0) && (delta.y == 0 || y + delta.y >= 0) {
                    x = x + delta.x;
                    y = y + delta.y;

                    let cell = self.get(x as usize, y as usize)?;
                    if op(cell) {
                        break Some(cell);
                    }
                } else {
                    break None;
                }
            }
        } else {
            None
        }
    }

    pub fn simulate<FR, FT>(&mut self, rule: FR, terminate: FT)
    where
        FR: Fn(&Self, Point, &Cell) -> Cell,
        FT: Fn(&Self, &Self) -> bool,
    {
        let mut next = self.clone();
        loop {
            for (i, cell) in self.cells.iter().enumerate() {
                next.cells[i] = rule(
                    &self,
                    Point {
                        x: (i % self.width) as isize,
                        y: (i / self.width) as isize,
                    },
                    cell,
                );
            }

            std::mem::swap(&mut self.cells, &mut next.cells);
            if terminate(self, &next) {
                break;
            }
        }
    }

    pub fn cell_count(&self, cell: &Cell) -> usize {
        self.cells.iter().filter(|&c| c == cell).count()
    }
}

fn parse(filename: &Path) -> Result<Grid, String> {
    let data = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 11: {}", err))?
        .lines()
        .map(|line| line.chars().map(Cell::try_from).collect::<Result<Vec<Cell>, _>>())
        .collect::<Result<Vec<Vec<Cell>>, _>>()?;

    let mut grid = Grid::new(data.len(), data[0].len());
    for (y, row) in data.iter().enumerate() {
        if row.len() != grid.width {
            Err(format!(
                "Not all rows have the same length: expected {}, found {}",
                grid.width,
                row.len()
            ))?
        }
        for (x, cell) in row.iter().enumerate() {
            *grid.get_mut(x, y).unwrap() = *cell;
        }
    }

    Ok(grid)
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let mut grid = parse(filename)?;

    grid.simulate(
        |grid, at, cell| match cell {
            Cell::Floor => Cell::Floor,
            Cell::Empty => {
                if grid.adjacent_next(&at).iter().any(|&cell| cell == Some(&Cell::Taken)) {
                    Cell::Empty
                } else {
                    Cell::Taken
                }
            }
            Cell::Taken => {
                if grid
                    .adjacent_next(&at)
                    .iter()
                    .filter(|&&cell| cell == Some(&Cell::Taken))
                    .count()
                    >= 4
                {
                    Cell::Empty
                } else {
                    Cell::Taken
                }
            }
        },
        |grid, prev| grid == prev,
    );

    Ok(format!(
        "Seats taken when stabilized: {}",
        grid.cell_count(&Cell::Taken)
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let mut grid = parse(filename)?;

    grid.simulate(
        |grid, at, cell| match cell {
            Cell::Floor => Cell::Floor,
            Cell::Empty => {
                if grid
                    .adjacent_nearest(&at, |&cell| cell != Cell::Floor)
                    .iter()
                    .any(|&cell| cell == Some(&Cell::Taken))
                {
                    Cell::Empty
                } else {
                    Cell::Taken
                }
            }
            Cell::Taken => {
                if grid
                    .adjacent_nearest(&at, |&cell| cell != Cell::Floor)
                    .iter()
                    .filter(|&&cell| cell == Some(&Cell::Taken))
                    .count()
                    >= 5
                {
                    Cell::Empty
                } else {
                    Cell::Taken
                }
            }
        },
        |grid, prev| grid == prev,
    );

    Ok(format!(
        "Seats taken when stabilized: {}",
        grid.cell_count(&Cell::Taken)
    ))
}
