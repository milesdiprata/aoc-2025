use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct BatteryBank {
    joltages: Vec<u8>,
}

impl FromStr for BatteryBank {
    type Err = Error;

    fn from_str(joltages: &str) -> Result<Self> {
        if joltages.is_empty() {
            return Err(anyhow!("empty joltage ratings"));
        }

        let joltages = joltages
            .chars()
            .map(|joltage| joltage.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("invalid joltage rating"))?;

        if joltages.len() < 12 {
            return Err(anyhow!("bank size less than 12"));
        }

        #[allow(clippy::cast_possible_truncation)]
        Ok(Self {
            joltages: joltages.into_iter().map(|joltage| joltage as u8).collect(),
        })
    }
}

impl BatteryBank {
    fn joltage_max(&self) -> u8 {
        fn dfs(joltages: &[u8], idx: usize, subset: &mut Vec<u8>, max: &mut u8) {
            if subset.len() == 2 {
                *max = (*max).max((10 * subset[0]) + subset[1]);
                return;
            }

            if idx == joltages.len() {
                return;
            }

            subset.push(joltages[idx]);
            dfs(joltages, idx + 1, subset, max);

            subset.pop();
            dfs(joltages, idx + 1, subset, max);
        }

        let mut subset = Vec::with_capacity(2 + 1);
        let mut max = 0;

        dfs(&self.joltages, 0, &mut subset, &mut max);

        max
    }

    fn joltage_max2(&self) -> u64 {
        let to_remove = self.joltages.len() - 12;

        let mut removed = 0usize;
        let mut stack = Vec::with_capacity(12);

        for &joltage in &self.joltages {
            while removed < to_remove && stack.last().is_some_and(|&last| last < joltage) {
                removed += 1;
                stack.pop();
            }

            stack.push(joltage);
        }

        stack.truncate(12);

        let mut max = 0;
        for joltage in stack {
            max = (10 * max) + u64::from(joltage);
        }

        max
    }
}

fn part1(banks: &[BatteryBank]) -> u64 {
    banks
        .iter()
        .map(BatteryBank::joltage_max)
        .map(u64::from)
        .sum()
}

fn part2(banks: &[BatteryBank]) -> u64 {
    banks
        .iter()
        .map(BatteryBank::joltage_max2)
        .map(u64::from)
        .sum()
}

fn main() -> Result<()> {
    let banks = BufReader::new(File::open("in/day3.txt")?)
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| line.parse::<BatteryBank>())
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&banks);
    let part2 = self::part2(&banks);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    assert_eq!(part1, 17_346);
    assert_eq!(part2, 172_981_362_045_136);

    Ok(())
}
