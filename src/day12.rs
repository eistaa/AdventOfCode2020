use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::str::FromStr;

enum Action {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    // -----
    Right(i32),
    Left(i32),
    // -----
    Forward(i32),
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().nth(0).unwrap() {
            'N' => Ok(Self::North(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'S' => Ok(Self::South(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'W' => Ok(Self::West(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'E' => Ok(Self::East(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'R' => Ok(Self::Right(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'L' => Ok(Self::Left(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            'F' => Ok(Self::Forward(
                i32::from_str(&value[1..]).map_err(|err| format!("Failed to parse number: {}", err))?,
            )),
            c => Err(format!("Unknown character: {}", c)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
struct Point {
    pub east: i32,
    pub north: i32,
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Self { east: p.0, north: p.1 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Ship {
    at: Point,
    waypoint: Point,
    heading: i32,
}

impl Ship {
    fn new(waypoint: Point, heading: i32) -> Self {
        Ship {
            at: (0, 0).into(),
            waypoint,
            heading,
        }
    }

    fn update_pt1(mut self, action: &Action) -> Self {
        match action {
            Action::North(dist) => self.at.north += dist,
            Action::South(dist) => self.at.north -= dist,
            Action::West(dist) => self.at.east -= dist,
            Action::East(dist) => self.at.east += dist,
            // -----
            Action::Right(deg) => self.heading = new_heading(self.heading, -*deg),
            Action::Left(deg) => self.heading = new_heading(self.heading, *deg),
            // -----
            Action::Forward(dist) => match &self.heading {
                0 => self.at.north += dist,
                90 => self.at.east -= dist,
                180 => self.at.north -= dist,
                270 => self.at.east += dist,
                _ => panic!("Non-90-degree increment heading"),
            },
        }

        self
    }

    fn update_pt2(mut self, action: &Action) -> Self {
        match action {
            Action::North(dist) => self.waypoint.north += dist,
            Action::South(dist) => self.waypoint.north -= dist,
            Action::West(dist) => self.waypoint.east -= dist,
            Action::East(dist) => self.waypoint.east += dist,
            // -----
            Action::Right(deg) => {
                self.waypoint = rotate_waypoint(self.waypoint, -*deg);
                self.heading = new_heading(self.heading, -*deg)
            }
            Action::Left(deg) => {
                self.waypoint = rotate_waypoint(self.waypoint, *deg);
                self.heading = new_heading(self.heading, *deg);
            }
            // -----
            Action::Forward(times) => {
                self.at.north += times * self.waypoint.north;
                self.at.east += times * self.waypoint.east;
            }
        }

        self
    }

    fn manhattan(&self) -> i32 {
        i32::abs(self.at.east) + i32::abs(self.at.north)
    }
}

fn new_heading(old: i32, update: i32) -> i32 {
    let mut new = old + update;
    loop {
        if new < 0 {
            new += 360;
            continue;
        } else if new >= 360 {
            new -= 360;
            continue;
        }

        break;
    }

    new
}

fn rotate_waypoint(mut waypoint: Point, rot: i32) -> Point {
    for _ in 0..i32::abs(rot / 90) {
        let tmp = waypoint.east;
        waypoint.east = -i32::signum(rot) * waypoint.north;
        waypoint.north = i32::signum(rot) * tmp;
    }

    waypoint
}

fn parse(filename: &Path) -> Result<Vec<Action>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 12: {}", err))?
        .split_ascii_whitespace()
        .map(Action::try_from)
        .collect()
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    let ship = Ship::new(Point::default(), 270);
    let ship = data.iter().fold(ship, |ship, action| ship.update_pt1(&action));

    Ok(format!("Ship have moved # manhattan distance: {}", ship.manhattan()))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    let ship = Ship::new((10, 1).into(), 270);
    let ship = data.iter().fold(ship, |ship, action| ship.update_pt2(&action));

    Ok(format!("Ship have moved # manhattan distance: {}", ship.manhattan()))
}
