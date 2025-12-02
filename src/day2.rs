use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct IdRange {
    range: RangeInclusive<usize>,
}

impl FromStr for IdRange {
    type Err = Error;

    fn from_str(range: &str) -> Result<Self> {
        let mut split = range.split('-');

        let first = split
            .next()
            .ok_or_else(|| anyhow!("missing first ID"))?
            .parse()?;
        let last = split
            .next()
            .ok_or_else(|| anyhow!("missing last ID"))?
            .parse()?;

        Ok(Self {
            range: (first..=last),
        })
    }
}

impl IdRange {
    fn invalid_ids(&self) -> Vec<usize> {
        let mut invalid_ids = Vec::new();

        for id in self.range.clone() {
            let str = id.to_string();
            if str.len() % 2 == 0 {
                let (first, last) = str.split_at(str.len() / 2);
                if first == last {
                    invalid_ids.push(id);
                }
            }
        }

        invalid_ids
    }
}

fn part1(ids: &[IdRange]) -> usize {
    ids.iter().flat_map(IdRange::invalid_ids).sum()
}

fn main() -> Result<()> {
    let ids = fs::read_to_string("in/day2.txt")?
        .split(',')
        .map(IdRange::from_str)
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&ids);

    println!("Part 1: {part1}");

    assert_eq!(part1, 18_952_700_150);

    Ok(())
}
