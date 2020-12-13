use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Notes {
    pub arrival: i64,
    pub departures: Vec<(i64, i64)>,
}

fn parse(filename: &Path) -> Result<Notes, String> {
    let mut lines: Vec<String> = fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 13: {}", err))?
        .lines()
        .take(2)
        .map(String::from)
        .collect();

    Ok(Notes {
        arrival: i64::from_str(lines.get(0).ok_or("Failed to read arrival".to_string())?)
            .map_err(|err| format!("Failed to parse arrival: {}", err))?,
        departures: lines
            .get(1)
            .ok_or("Failed to read busses".to_string())?
            .split(",")
            .enumerate()
            .filter(|&(i, d)| d != "x")
            .map(|(i, d)| {
                let bus = i64::from_str(d).map_err(|err| format!("Failed to parse bus: {}", err))?;
                Ok((bus, bus - (i as i64)))
            })
            .collect::<Result<Vec<(i64, i64)>, String>>()?,
    })
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let notes = parse(filename)?;

    let earliest = notes
        .departures
        .iter()
        .map(|(d, _)| {
            let multiple = notes.arrival / d;
            if multiple * d < notes.arrival {
                (*d, (multiple + 1) * d)
            } else {
                (*d, multiple * d)
            }
        })
        .min_by(|tup1, tup2| tup1.1.cmp(&tup2.1))
        .unwrap();

    Ok(format!(
        "Earliest departure after arrival at:{}",
        earliest.0 * (earliest.1 - notes.arrival)
    ))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let notes = dbg!(parse(filename)?);

    dbg!(chinese_remainder(
        &notes.departures.iter().map(|(m, r)| *r).collect::<Vec<i64>>(),
        &notes.departures.iter().map(|(m, r)| *m).collect::<Vec<i64>>()
    ));

    Ok("".to_string())
}

// all below are from Rosetta code: http://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
