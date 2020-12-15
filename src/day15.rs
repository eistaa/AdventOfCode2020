use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn parse(filename: &Path) -> Result<Vec<usize>, String> {
    fs::read_to_string(filename)
        .map_err(|err| format!("Failed to read data for day 15: {}", err))?
        .trim()
        .split(",")
        .map(|num| usize::from_str(num).map_err(|err| format!("Failed to parse number: {}", err)))
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Times {
    Never,
    Once(usize),
    Twice(usize, usize),
}

impl Times {
    fn update(self, turn: usize) -> Self {
        match self {
            Self::Never => Self::Once(turn),
            Self::Once(prev) | Self::Twice(_, prev) => Self::Twice(prev, turn),
        }
    }
}

#[derive(Debug)]
struct NumberSayer {
    pub turn: usize,
    pub last: usize,
    memory: HashMap<usize, Times>,
}

impl NumberSayer {
    fn new(starting: &[usize]) -> Self {
        let mut sayer = NumberSayer {
            turn: 0,
            last: 0,
            memory: HashMap::new(),
        };

        for (turn, num) in starting.iter().enumerate() {
            sayer.update_memory(turn + 1, *num);
        }

        sayer
    }

    fn update_memory(&mut self, turn: usize, last: usize) {
        self.last = last;
        self.turn = turn;
        self.memory
            .entry(last)
            .and_modify(|e| *e = e.update(turn))
            .or_insert(Times::Once(turn));
    }

    fn take_turn(&mut self) -> &Self {
        self.update_memory(
            self.turn + 1,
            match self.memory.get(&self.last).unwrap_or(&Times::Never) {
                Times::Never => self.last,
                Times::Once(_) => 0,
                Times::Twice(first, second) => *second - *first,
            },
        );

        self
    }
}

pub fn part01(filename: &Path) -> Result<String, String> {
    let mut sayer = NumberSayer::new(&parse(filename)?);
    while sayer.take_turn().turn < 2020 {}

    Ok(format!("{}", sayer.last))
}

pub fn part02(filename: &Path) -> Result<String, String> {
    let mut sayer = NumberSayer::new(&parse(filename)?);
    while sayer.take_turn().turn < 30_000_000 {}

    Ok(format!("{}", sayer.last))
}
