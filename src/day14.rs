use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

#[derive(Clone, Debug)]
struct MaskedValue {
    pub mask: usize,
    pub value: usize,
    pub values: Vec<(usize, usize)>,
}

impl MaskedValue {
    fn new(mask: usize, value: usize) -> Self {
        Self {
            mask,
            value,
            values: Vec::new(),
        }
    }
}

fn parse(filename: &Path) -> Result<Vec<MaskedValue>, String> {
    let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    let mut data = Vec::new();
    let mut value = None;

    for line in fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 14: \"{}\"", err))?
        .lines()
    {
        match &line[..4] {
            "mask" => {
                if value.is_some() {
                    data.push(value.take().unwrap());
                }

                value = Some(MaskedValue::new(
                    // NB: this value is negated!
                    !usize::from_str_radix(&line[7..].replace("1", "0").replace("X", "1"), 2)
                        .map_err(|err| format!("Failed to parse mask: \"{}\"", err))?,
                    usize::from_str_radix(&line[7..].replace("X", "0"), 2)
                        .map_err(|err| format!("Failed to parse mask value: \"{}\"", err))?,
                ));

                Ok(())
            }
            "mem[" => {
                let data = re
                    .captures(line)
                    .ok_or(format!("Failed to parse memory line: \"{}\"", line))?;
                value.get_or_insert_with(|| unreachable!()).values.push((
                    usize::from_str(data.get(1).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse memory address: \"{}\"", err))?,
                    usize::from_str(data.get(2).unwrap().as_str())
                        .map_err(|err| format!("Failed to parse memory value: \"{}\"", err))?,
                ));
                Ok(())
            }
            _ => Err(format!("Unknown line: \"{}\"", line)),
        }?
    }

    if value.is_some() {
        data.push(value.take().unwrap());
    }

    Ok(data)
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let program = parse(filename)?;

    let mut memory = HashMap::new();
    for mv in program.iter() {
        for (addr, val) in mv.values.iter() {
            memory.insert(addr, (val & !mv.mask) | mv.value);
        }
    }

    Ok(format!("Sum of memory: {}", memory.values().sum::<usize>()))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let program = parse(filename)?;

    let mut memory = HashMap::new();
    for mv in program.iter() {
        let floatmask = !mv.mask & !mv.value; // 1 where there is a floating bit
        let ones = floatmask.count_ones(); // number of floating bits

        // for each possible bit combination for the floating bits
        for mut pattern in 0..usize::pow(2, ones) {
            let mut floating = mv.value; // accumulator for the floating bit pattern
            let mut i = 1; // counter for the floating bit

            // for each bit in the bit combination
            while pattern != 0 {
                // locate the bit to set
                while i != (1usize << 36) {
                    if floatmask & i > 0 {
                        // found the bit, retrieve the correct bit and from pattern and set it in floating
                        floating |= (pattern & 1) << i.trailing_zeros();
                        i <<= 1;
                        break;
                    }
                    i <<= 1;
                }
                pattern >>= 1;
            }

            let tmp2 = mv.mask & !mv.value;
            for (addr, val) in mv.values.iter() {
                // rules, the rules correspond to each of the three or-ed parts:
                //  1. where the mask bit is 0: pass through the address bit
                //  2. where the mask bit is 1: set address bit to 1
                //  3. where the mask bit is floating: use the floating bit-pattern value
                memory.insert((addr & tmp2) | floating, *val);
            }
        }
    }

    Ok(format!("Sum of memory: {}", memory.values().sum::<usize>()))
}
