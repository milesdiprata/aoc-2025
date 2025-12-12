use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Light {
    Off,
    On,
}

#[derive(Debug)]
struct Machine {
    diagram: Vec<Light>,
    buttons: Vec<Vec<usize>>,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(machine: &str) -> Result<Self> {
        let mut machine = machine.split_ascii_whitespace().peekable();

        let diagram = machine
            .next()
            .ok_or_else(|| anyhow!("missing light diagram"))?;
        let diagram = diagram
            .get(1..diagram.len() - 1)
            .ok_or_else(|| anyhow!("light diagram is empty"))?
            .chars()
            .map(Light::new)
            .collect::<Result<Vec<_>>>()?;

        let mut buttons = Vec::new();
        while machine.peek().is_some_and(|&next| next.starts_with('(')) {
            let schematic = machine.next().unwrap();
            let schematic = schematic
                .get(1..schematic.len() - 1)
                .ok_or_else(|| anyhow!("button schematic is empty"))?
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?;

            buttons.push(schematic);
        }

        Ok(Self { diagram, buttons })
    }
}

impl Light {
    fn new(state: char) -> Result<Self> {
        match state {
            '.' => Ok(Self::Off),
            '#' => Ok(Self::On),
            _ => Err(anyhow!(format!("unknown light state '{state}'"))),
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl Machine {
    fn configure(&self) -> usize {
        fn dfs(
            diagram: &[Light],
            buttons: &[Vec<usize>],
            start: usize,
            presses: usize,
            presses_min: usize,
            state: &mut [Light],
        ) -> usize {
            if state == diagram {
                return presses;
            }

            if presses > presses_min {
                return presses;
            }

            let mut presses_min = presses_min;

            for i in start..buttons.len() {
                for &b in &buttons[i] {
                    state[b] = state[b].toggle();
                }

                presses_min = presses_min.min(dfs(
                    diagram,
                    buttons,
                    i + 1,
                    presses + 1,
                    presses_min,
                    state,
                ));

                for &b in &buttons[i] {
                    state[b] = state[b].toggle();
                }
            }

            presses_min
        }

        let mut state = vec![Light::Off; self.diagram.len()];
        dfs(&self.diagram, &self.buttons, 0, 0, usize::MAX, &mut state)
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(Machine::configure).sum()
}

fn main() -> Result<()> {
    let machines = BufReader::new(File::open("in/day10.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| Machine::from_str(&line))
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&machines);

    println!("Part 1: {part1}");

    assert_eq!(part1, 417);

    Ok(())
}
