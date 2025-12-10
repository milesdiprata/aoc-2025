use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
struct JunctionBox {
    coords: [i64; 3],
}

impl FromStr for JunctionBox {
    type Err = Error;

    fn from_str(coords: &str) -> Result<Self> {
        let mut coords = coords.split(',');

        let x = coords
            .next()
            .ok_or_else(|| anyhow!("missing x-coordinate"))?
            .parse()?;
        let y = coords
            .next()
            .ok_or_else(|| anyhow!("missing y-coordinate"))?
            .parse()?;
        let z = coords
            .next()
            .ok_or_else(|| anyhow!("missing z-coordinate"))?
            .parse()?;

        Ok(Self { coords: [x, y, z] })
    }
}

impl JunctionBox {
    fn dist(&self, other: &Self) -> i64 {
        self.coords
            .iter()
            .zip(&other.coords)
            .map(|(coord_i, coord_j)| coord_i - coord_j)
            .map(|delta| delta.pow(2))
            .sum()
    }
}

fn part1(boxes: &[JunctionBox]) -> usize {
    fn find(parents: &mut [usize], i: usize) -> usize {
        if parents[i] != i {
            parents[i] = find(parents, parents[i]);
        }

        parents[i]
    }

    let mut pairs = Vec::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            pairs.push((i, j, boxes[i].dist(&boxes[j])));
        }
    }

    pairs.select_nth_unstable_by_key(999, |&(_, _, dist)| dist);

    let shortest = &pairs[..1000];
    let mut parents = (0..boxes.len()).collect::<Vec<_>>();

    for &(i, j, _) in shortest {
        let root_i = find(&mut parents, i);
        let root_j = find(&mut parents, j);

        if root_i != root_j {
            parents[root_i] = root_j;
        }
    }

    let mut lens = HashMap::<usize, usize>::new();
    for i in 0..boxes.len() {
        *lens.entry(find(&mut parents, i)).or_default() += 1;
    }

    let mut lens = lens.into_values().collect::<Vec<_>>();
    lens.select_nth_unstable_by(2, |i, j| j.cmp(i));

    lens.into_iter().take(3).product()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day8.txt")?;
    let boxes = input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<Result<Vec<_>>>()?;

    let part1 = self::part1(&boxes);

    println!("Part 1: {part1}");

    assert_eq!(part1, 129_564);

    Ok(())
}
