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
    joltages: Vec<usize>,
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

        let joltages = machine
            .next()
            .ok_or_else(|| anyhow!("missing joltage requirements"))?;
        let joltages = joltages
            .get(1..joltages.len() - 1)
            .ok_or_else(|| anyhow!("joltage requirements are empty"))?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            diagram,
            buttons,
            joltages,
        })
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

    const fn toggle(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl Machine {
    fn configure_lights(&self) -> usize {
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

            if presses >= presses_min {
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

    fn configure_joltages(&self) -> Result<usize> {
        use z3::ast::Int;
        use z3::Optimize;
        use z3::SatResult;

        // Creates Z3 optimizer
        let opt = Optimize::new();

        // Creates integer variables for each button
        // How many times to press
        let x = (0..self.buttons.len())
            .map(|i| Int::new_const(format!("x{i}")))
            .collect::<Vec<Int>>();

        // Constraint 1: non-negative number of button presses
        let zero = Int::from_i64(0);
        for var in &x {
            opt.assert(&var.ge(&zero));
        }

        // Constraint 2: sum of contributing buttons equals target joltage
        for i in 0..self.joltages.len() {
            let target = Int::from_i64(i64::try_from(self.joltages[i])?);

            // Finds all buttons that affect this joltage
            let contributing = self
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(b, buttons)| buttons.contains(&i).then_some(&x[b]))
                .collect::<Vec<_>>();

            if contributing.is_empty() {
                // No buttons affect this joltage, target must be zero
                if self.joltages[i] != 0 {
                    return Err(anyhow!("unsolvable"));
                }
            } else {
                // Sum of all contributing button presses must equal target
                let sum = Int::add(&contributing);
                opt.assert(&sum.eq(&target));
            }
        }

        // Objective: minimize total number of button presses
        let total = Int::add(&x);
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt
                    .get_model()
                    .ok_or_else(|| anyhow!("model does not exist"))?;

                let presses = x
                    .iter()
                    .filter_map(|var| model.eval(var, true))
                    .map(|val| val.as_i64())
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| anyhow!("value is not i64"))?
                    .into_iter()
                    .map(usize::try_from)
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .sum();

                Ok(presses)
            }
            SatResult::Unsat => Err(anyhow!("no solution exists for machine")),
            SatResult::Unknown => Err(anyhow!("solver returned unknown")),
        }
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(Machine::configure_lights).sum()
}

fn part2(machines: &[Machine]) -> Result<usize> {
    let presses = machines
        .iter()
        .map(Machine::configure_joltages)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum();

    Ok(presses)
}

fn main() -> Result<()> {
    let machines = BufReader::new(File::open("in/day10.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| Machine::from_str(&line))
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&machines);
    let part2 = self::part2(&machines)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 417);
    assert_eq!(part2, 16_765);

    Ok(())
}
