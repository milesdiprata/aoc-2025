use std::cmp::Ordering;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Ingredient {
    id: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct FreshIngredient {
    range: RangeInclusive<Ingredient>,
}

impl PartialOrd for FreshIngredient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FreshIngredient {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.range.start() == other.range.start() {
            self.range.end().cmp(other.range.end())
        } else {
            self.range.start().cmp(other.range.start())
        }
    }
}

impl FromStr for Ingredient {
    type Err = Error;

    fn from_str(id: &str) -> Result<Self> {
        Ok(Self { id: id.parse()? })
    }
}

impl FromStr for FreshIngredient {
    type Err = Error;

    fn from_str(range: &str) -> Result<Self> {
        let mut split = range.split('-');

        let start = split
            .next()
            .ok_or_else(|| anyhow!("missing fresh ingredient start ID"))?
            .parse()?;
        let end = split
            .next()
            .ok_or_else(|| anyhow!("missing fresh ingredient end ID"))?
            .parse()?;

        Ok(Self { range: start..=end })
    }
}

fn merge_fresh_ranges(mut fresh: Vec<FreshIngredient>) -> Vec<FreshIngredient> {
    fresh.sort();

    let mut merged = if let Some(first) = fresh.first().cloned() {
        vec![first]
    } else {
        return fresh;
    };

    for fresh in fresh.into_iter().skip(1) {
        let (start, end) = fresh.range.into_inner();
        let (start_prev, end_prev) = merged.pop().unwrap().range.into_inner();

        if start <= end_prev {
            merged.push(FreshIngredient {
                range: start_prev..=end_prev.max(end),
            });
        } else {
            merged.push(FreshIngredient {
                range: { start_prev..=end_prev },
            });
            merged.push(FreshIngredient { range: start..=end });
        }
    }

    merged
}

fn part1(fresh: &[FreshIngredient], avail: &[Ingredient]) -> usize {
    let mut fresh_count = 0;

    for avail in avail {
        if fresh.iter().any(|fresh| fresh.range.contains(avail)) {
            fresh_count += 1;
        }
    }

    fresh_count
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day5.txt")?;
    let mut split = input.split("\n\n");

    let fresh = split
        .next()
        .ok_or_else(|| anyhow!("missing fresh ingredient ranges"))?
        .lines()
        .map(FreshIngredient::from_str)
        .collect::<Result<Vec<_>>>()?;
    let avail = split
        .next()
        .ok_or_else(|| anyhow!("missing available ingredient IDs"))?
        .lines()
        .map(Ingredient::from_str)
        .collect::<Result<Vec<_>>>()?;

    let fresh = self::merge_fresh_ranges(fresh);

    let part1 = self::part1(&fresh, &avail);

    println!("Part 1: {part1}");

    assert_eq!(part1, 674);

    Ok(())
}
