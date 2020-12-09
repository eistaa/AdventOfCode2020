use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse(filename: &Path) -> Result<Vec<Instruction>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 08: {}", err))?
        .lines()
        .map(|line| match &line[..3] {
            "acc" => {
                let value =
                    i32::from_str(&line[4..]).map_err(|err| format!("Failed to parse ACC increment: {}", err))?;
                Ok(Instruction::Acc(value))
            }
            "jmp" => {
                let value =
                    i32::from_str(&line[4..]).map_err(|err| format!("Failed to parse JMP increment: {}", err))?;
                Ok(Instruction::Jmp(value))
            }
            "nop" => {
                let value =
                    i32::from_str(&line[4..]).map_err(|err| format!("Failed to parse NOP increment: {}", err))?;
                Ok(Instruction::Nop(value))
            }
            e => Err(format!("Unknown instruction: {}", e)),
        })
        .collect::<Result<Vec<Instruction>, String>>()
}

fn interpreter(program: &Vec<Instruction>) -> Result<i32, i32> {
    let mut accumulator = 0;

    let mut executed = HashSet::new();
    let mut pc: i32 = 0;

    loop {
        if pc as usize == program.len() {
            break Ok(accumulator);
        } else if executed.contains(&pc) {
            break Err(accumulator);
        }

        executed.insert(pc);
        let instruction = *&program[pc as usize];
        pc = pc + 1;

        match instruction {
            Instruction::Acc(v) => accumulator = accumulator + v,
            Instruction::Jmp(v) => pc = pc + v - 1,
            Instruction::Nop(_) => continue,
        }
    }
}

fn replace_instruction(program: &Vec<Instruction>, idx: usize, instruction: &Instruction) -> Vec<Instruction> {
    Vec::from_iter(
        program[..idx]
            .iter()
            .chain(&[*instruction])
            .chain(&program[(idx + 1)..])
            .copied(),
    )
}

fn find_correction(program: &Vec<Instruction>) -> Result<i32, String> {
    for (i, instruction) in program.iter().enumerate() {
        match instruction {
            Instruction::Jmp(v) => {
                if let Ok(accumulator) = interpreter(&replace_instruction(&program, i, &Instruction::Nop(*v))) {
                    return Ok(accumulator);
                }
            }
            Instruction::Nop(v) => {
                if let Ok(accumulator) = interpreter(&replace_instruction(&program, i, &Instruction::Jmp(*v))) {
                    return Ok(accumulator);
                }
            }
            _ => (),
        }
    }

    Err("Could not find correction...".to_string())
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let program = parse(filename)?;

    match interpreter(&program) {
        Ok(acc) => Err(format!("Terminated successfully with accumulator: {}", acc)),
        Err(acc) => Ok(format!("Accumulator when program detects loop: {}", acc)),
    }
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let program = parse(filename)?;
    Ok(format!(
        "Found correction, accumulator after termination is: {}",
        find_correction(&program)?
    ))
}
