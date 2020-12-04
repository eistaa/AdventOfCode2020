use regex::Regex;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use lazy_static::lazy_static;

#[derive(Clone, Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn year_in_range(year: &str, min: i32, max: i32) -> bool {
    i32::from_str(year).map(|year| min <= year && year <= max) == Ok(true)
}

impl Passport {
    fn valid_byr(&self) -> bool {
        self.byr.as_ref().map(|year| year_in_range(year, 1920, 2002)) == Some(true)
    }

    fn valid_iyr(&self) -> bool {
        self.iyr.as_ref().map(|year| year_in_range(year, 2010, 2020)) == Some(true)
    }

    fn valid_eyr(&self) -> bool {
        self.eyr.as_ref().map(|year| year_in_range(year, 2020, 2030)) == Some(true)
    }

    fn valid_hgt(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }

        self.hgt.as_ref().map(|hgt| {
            RE.captures(hgt).map(|groups| {
                let height = i32::from_str(groups.get(1).unwrap().as_str()).unwrap();
                let is_cm = groups.get(2).unwrap().as_str() == "cm";

                (is_cm && 150 <= height && height <= 193) || (!is_cm && 59 <= height && height <= 76)
            }) == Some(true)
        }) == Some(true)
    }

    fn valid_hcl(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        self.hcl.as_ref().map(|hcl| RE.is_match(hcl)) == Some(true)
    }

    fn valid_ecl(&self) -> bool {
        self.ecl.as_ref().map(|ecl| {
            ecl == "amb" || ecl == "blu" || ecl == "brn" || ecl == "gry" || ecl == "grn" || ecl == "hzl" || ecl == "oth"
        }) == Some(true)
    }

    fn valid_pid(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }

        self.pid.as_ref().map(|pid| RE.is_match(pid)) == Some(true)
    }
}

fn parse(filename: &Path) -> Result<Vec<Passport>, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]{3}):(.*)$").unwrap();
    }

    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 03: {}", err))?
        .split("\n\n")
        .map(|text| {
            let mut passport = Passport::default();

            for field in text.split_ascii_whitespace() {
                let fields = RE
                    .captures(field)
                    .ok_or(format!("Failed to parse passport field: {}", field))?;
                let key = fields.get(1).unwrap().as_str();
                let value = fields.get(2).unwrap().as_str();

                match key {
                    "byr" => passport.byr = Some(value.to_string()),
                    "iyr" => passport.iyr = Some(value.to_string()),
                    "eyr" => passport.eyr = Some(value.to_string()),
                    "hgt" => passport.hgt = Some(value.to_string()),
                    "hcl" => passport.hcl = Some(value.to_string()),
                    "ecl" => passport.ecl = Some(value.to_string()),
                    "pid" => passport.pid = Some(value.to_string()),
                    "cid" => passport.cid = Some(value.to_string()),
                    _ => unreachable!(),  // don't handle since the data doesn't include extra fields
                }
            }

            Ok(passport)
        })
        .collect::<Result<Vec<Passport>, String>>()
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    Ok(format!(
        "Valid passports {} (total: {})",
        data.iter()
            .filter(|passport| {
                passport.byr.is_some()
                    && passport.iyr.is_some()
                    && passport.eyr.is_some()
                    && passport.hgt.is_some()
                    && passport.hcl.is_some()
                    && passport.ecl.is_some()
                    && passport.pid.is_some()
            })
            .count(),
        data.len()
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let data = parse(filename)?;

    Ok(format!(
        "Valid passports {} (total: {})",
        data.iter()
            .filter(|passport| {
                passport.valid_byr()
                    && passport.valid_iyr()
                    && passport.valid_eyr()
                    && passport.valid_hgt()
                    && passport.valid_hcl()
                    && passport.valid_ecl()
                    && passport.valid_pid()
            })
            .count(),
        data.len()
    ))
}
